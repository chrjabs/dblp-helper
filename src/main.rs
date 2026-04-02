use std::io::Write;

use clap::Parser;
use cli::{Color, CommonGetArgs, DblpServerArgs, GetAllArgs, GetArgs, SearchArgs};
use color_eyre::eyre::{Result, WrapErr, bail};
use dblp::Record;
use futures::{StreamExt, TryStreamExt, stream};
use owo_colors::OwoColorize;

mod bibtex;
mod cli;
mod dblp;
mod fixers;
mod latex;
mod serde_utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = cli::Args::parse();
    args.color.init();

    match args.command {
        cli::Commands::Search(search_args) => search(search_args, args.dblp, args.color).await,
        cli::Commands::Get(get_args) => get(get_args, args.dblp, args.color).await,
        cli::Commands::GetAll(get_all_args) => get_all(get_all_args, args.dblp, args.color).await,
    }
}

async fn search(args: SearchArgs, dblp: DblpServerArgs, color: Color) -> Result<()> {
    let mut query = dblp::search::Query::new(args.r#type, &args.query);
    if let Some(hits) = args.hits {
        query.hits(hits);
    }

    let response = query.get(&dblp).await?;

    for hit in response.iter_hits() {
        let mut display = hit.display();
        if color.should_color(&std::io::stdout()) {
            display.colorize();
        }
        println!();
        println!("{display}");
    }

    Ok(())
}

fn fixup(rec: &mut dblp::Record, args: &CommonGetArgs) {
    fixers::author_num(rec);
    fixers::escape_latex(rec);
    fixers::page_range(rec);
    fixers::names(rec);
    fixers::strip_title_period(rec);
    fixers::capital_after_colon(rec);
    fixers::proper_nouns(rec);
    fixers::acronyms(rec);
    fixers::weird_urls(rec);
    fixers::date_ranges(rec);
    fixers::dashes(rec);
    fixers::manually_correct(rec);
    if !args.unicode {
        fixers::unicode(rec);
    }
    if !args.all_externals {
        fixers::single_external(rec);
    }
}

async fn get(args: GetArgs, dblp: DblpServerArgs, color: Color) -> Result<()> {
    let mut rec = dblp::Record::get(
        &args.key,
        !args.common.crossref,
        !args.common.dont_expand_journals,
        &dblp,
    )
    .await?;
    fixup(&mut rec, &args.common);
    let crossref = if let Some(key) = rec.crossref_key() {
        let mut crossref = dblp::Record::get(key, !args.common.crossref, false, &dblp).await?;
        fixup(&mut crossref, &args.common);
        fixers::expand_booktitle(&mut rec, &crossref);
        Some(crossref)
    } else {
        None
    };
    let mut bibtex = rec.bibtex();
    if color.should_color(&std::io::stdout()) {
        bibtex.colorize();
    }
    println!("{bibtex}");
    if let Some(rec) = crossref {
        println!();
        let mut bibtex = rec.bibtex();
        if color.should_color(&std::io::stdout()) {
            bibtex.colorize();
        }
        println!("{bibtex}");
    }
    Ok(())
}

enum FetchRes {
    Rec(dblp::Record),
    Unknown(String),
}

async fn fetch_record<Service>(
    key: &str,
    dblp: &DblpServerArgs,
    service: &mut Service,
    opts: &cli::CommonGetArgs,
) -> Result<FetchRes, dblp::record::Error>
where
    Service: tower::Service<
            reqwest::Request,
            Response = reqwest::Response,
            Error = Box<dyn std::error::Error + Send + std::marker::Sync + 'static>,
        >,
{
    let mut rec = match dblp::Record::get_with_service(
        key,
        !opts.crossref,
        !opts.dont_expand_journals,
        dblp,
        service,
    )
    .await
    {
        Ok(rec) => rec,
        Err(err) => match err {
            dblp::record::Error::UnknownKey(key) => return Ok(FetchRes::Unknown(key)),
            err => return Err(err),
        },
    };
    fixup(&mut rec, opts);
    Ok(FetchRes::Rec(rec))
}

async fn fetch_keys<Service>(
    keys: &[String],
    dblp: &DblpServerArgs,
    service: &mut Service,
    opts: &cli::GetAllArgs,
    color: Color,
) -> Result<Vec<FetchRes>>
where
    Service: tower::Service<
            reqwest::Request,
            Response = reqwest::Response,
            Error = Box<dyn std::error::Error + Send + std::marker::Sync + 'static>,
        > + Clone,
{
    // Setup progress information
    let err_styles = {
        let mut styles = cli::Styles::default();
        if color.should_color(&std::io::stderr()) {
            styles.colorize();
        }
        styles
    };
    eprintln!("{}", "fetching DBLP records".style(err_styles.info));
    let bar = if color.should_color(&std::io::stderr()) {
        let bar = indicatif::ProgressBar::new(u64::try_from(keys.len())?);
        bar.set_style(indicatif::ProgressStyle::with_template(
            cli::Styles::GET_ALL_PROGRESS_TEMPLATE,
        )?);
        Some(bar)
    } else {
        None
    };

    let results: Vec<_> = stream::iter(keys)
        .map(|key| {
            let mut service = service.clone();
            let dblp = &dblp;
            if let Some(bar) = &bar {
                bar.set_message(key.clone());
            }
            let res = async move {
                fetch_record(key, dblp, &mut service, &opts.common)
                    .await
                    .wrap_err_with(|| format!("Failed to fetch record `{key}`"))
            };
            if let Some(bar) = &bar {
                bar.inc(1);
            }
            res
        })
        .buffered(dblp.concurrent_requests)
        .try_collect()
        .await?;

    Ok(results)
}

async fn get_all(mut args: GetAllArgs, dblp: DblpServerArgs, color: Color) -> Result<()> {
    args.latex_path.set_extension("aux");
    let keys: Result<Vec<_>, _> =
        latex::CiteKeyIter::new(&args.latex_path, !args.no_follow_inputs)?
            .filter(|res| {
                res.as_ref().is_err() || res.as_ref().is_ok_and(|key| key.starts_with("DBLP:"))
            })
            .collect();
    let mut keys = keys?;
    keys.sort_unstable();
    keys.dedup();

    let mut records = if !args.dont_reuse_existing
        && let Some(bibtex_path) = &args.bibtex_path
    {
        let content = std::fs::read_to_string(bibtex_path)?;
        bibtex::parse(&content)?
    } else {
        vec![]
    };
    records.sort_unstable_by(|a, b| a.key().cmp(b.key()));
    for rec in &mut records {
        fixup(rec, &args.common);
    }

    // Remove keys that are already present
    let mut idx = 0;
    keys.retain(|key| {
        let key = key.strip_prefix("DBLP:").unwrap_or(key);
        while idx < records.len() && *records[idx].key() < *key {
            idx += 1;
        }
        if idx < records.len() && *records[idx].key() == *key {
            return false;
        }
        true
    });

    let mut service = dblp::new_service(&dblp);

    let results = fetch_keys(&keys, &dblp, &mut service, &args, color).await?;

    let mut unknown_keys = String::new();
    let mut crossref_keys = vec![];

    for rec in &records {
        if let Some(key) = rec.crossref_key() {
            crossref_keys.push(key.to_owned());
        }
    }

    records.extend(results.into_iter().filter_map(|res| match res {
        FetchRes::Rec(rec) => {
            if let Some(key) = rec.crossref_key() {
                crossref_keys.push(key.to_owned());
            }
            Some(rec)
        }
        FetchRes::Unknown(key) => {
            unknown_keys.push_str("\n- ");
            unknown_keys.push_str(&key);
            None
        }
    }));

    crossref_keys.sort_unstable();
    crossref_keys.dedup();

    let mut crossref_recs = Vec::with_capacity(crossref_keys.len());

    // remove crossref keys we already downloaded
    let mut idx = 0;
    crossref_keys.retain(|key| {
        while idx < records.len() && *records[idx].key() < *key.as_str() {
            idx += 1;
        }
        if idx < records.len() && *records[idx].key() == *key {
            crossref_recs.push(records.remove(idx));
            return false;
        }
        true
    });

    let results = fetch_keys(&crossref_keys, &dblp, &mut service, &args, color).await?;

    crossref_recs.extend(results.into_iter().filter_map(|res| match res {
        FetchRes::Rec(record) => Some(record),
        FetchRes::Unknown(key) => {
            unknown_keys.push_str("\n- ");
            unknown_keys.push_str(&key);
            None
        }
    }));

    crossref_recs.sort_unstable_by(|a, b| a.key().cmp(b.key()));

    // extend booktitles from crossref
    if args.common.crossref {
        for rec in &mut records {
            if let Some(key) = rec.crossref_key() {
                // TODO: can probably do something more efficient here
                let Ok(idx) = crossref_recs.binary_search_by_key(&key, Record::key) else {
                    bail!("crossref key not found: {key}");
                };
                fixers::expand_booktitle(rec, &crossref_recs[idx]);
            }
        }
    }

    if let Some(bibtex_path) = &args.bibtex_path {
        let mut writer = std::fs::File::create(bibtex_path)?;
        for (idx, rec) in records.into_iter().chain(crossref_recs).enumerate() {
            if idx > 0 {
                writeln!(writer)?;
            }
            let bibtex = rec.bibtex();
            writeln!(writer, "{bibtex}")?;
        }
    } else {
        for (idx, rec) in records.into_iter().chain(crossref_recs).enumerate() {
            if idx > 0 {
                println!();
            }
            let mut bibtex = rec.bibtex();
            if color.should_color(&std::io::stdout()) {
                bibtex.colorize();
            }
            println!("{bibtex}");
        }
    }

    if !unknown_keys.is_empty() {
        bail!("unknown DBLP keys:{unknown_keys}");
    }
    Ok(())
}

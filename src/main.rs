use std::{fs, io};

use clap::Parser;
use cli::{Color, CommonGetArgs, DblpServerArgs, GetAllArgs, GetArgs, SearchArgs};
use color_eyre::eyre::{Result, bail};
use futures::{StreamExt, TryStreamExt, stream};
use owo_colors::OwoColorize;

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
    fixers::page_range(rec);
    fixers::names(rec);
    fixers::acronyms(rec);
    fixers::weird_urls(rec);
    if !args.unicode {
        fixers::unicode(rec);
    }
}

async fn get(args: GetArgs, dblp: DblpServerArgs, color: Color) -> Result<()> {
    let mut rec = dblp::Record::get(&args.key, &dblp).await?;
    fixup(&mut rec, &args.common);
    let mut bibtex = rec.bibtex();
    if color.should_color(&std::io::stdout()) {
        bibtex.colorize();
    }
    println!("{bibtex}");
    Ok(())
}

enum FetchRes {
    Rec(dblp::Record),
    Unknown(String),
}

async fn fetch_record(
    key: &str,
    dblp: &DblpServerArgs,
    client: &reqwest::Client,
    opts: &cli::CommonGetArgs,
) -> Result<FetchRes, dblp::record::Error> {
    let mut rec = match dblp::Record::get_with_client(key, dblp, client).await {
        Ok(rec) => rec,
        Err(err) => match err {
            dblp::record::Error::UnknownKey(key) => return Ok(FetchRes::Unknown(key)),
            err => return Err(err),
        },
    };
    fixup(&mut rec, opts);
    Ok(FetchRes::Rec(rec))
}

async fn get_all(mut args: GetAllArgs, dblp: DblpServerArgs, color: Color) -> Result<()> {
    args.path.set_extension("aux");
    let keys: Result<Vec<_>, _> =
        latex::CiteKeyIter::new(io::BufReader::new(fs::File::open(args.path)?))
            .filter(|res| res.as_ref().is_ok_and(|key| key.starts_with("DBLP:")))
            .collect();
    let mut keys = keys?;
    keys.sort_unstable();
    keys.dedup();

    let client = reqwest::Client::new();

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
            let client = &client;
            let dblp = &dblp;
            if let Some(bar) = &bar {
                bar.set_message(key.clone());
            }
            let res = async move { fetch_record(&key, dblp, client, &args.common).await };
            if let Some(bar) = &bar {
                bar.inc(1);
            }
            res
        })
        .buffered(args.concurrent_requests)
        .try_collect()
        .await?;

    let mut unknown_keys = String::new();
    let mut printed = false;

    for res in results {
        match res {
            FetchRes::Rec(rec) => {
                if printed {
                    println!();
                }
                let mut bibtex = rec.bibtex();
                if color.should_color(&std::io::stdout()) {
                    bibtex.colorize();
                }
                println!("{bibtex}");
                printed = true;
            }
            FetchRes::Unknown(key) => {
                unknown_keys.push_str("\n- ");
                unknown_keys.push_str(&key);
            }
        }
    }

    if !unknown_keys.is_empty() {
        bail!("unknown DBLP keys:{unknown_keys}");
    }
    Ok(())
}

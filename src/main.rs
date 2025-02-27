use std::{fs, io};

use clap::Parser;
use cli::{Color, CommonGetArgs, GetAllArgs, GetArgs, SearchArgs};
use color_eyre::eyre::Result;

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
        cli::Commands::Search(search_args) => search(search_args, args.color).await,
        cli::Commands::Get(get_args) => get(get_args, args.color).await,
        cli::Commands::GetAll(get_all_args) => get_all(get_all_args, args.color).await,
    }
}

async fn search(args: SearchArgs, color: Color) -> Result<()> {
    let mut query = dblp::search::Query::new(args.r#type, &args.query);
    if let Some(hits) = args.hits {
        query.hits(hits);
    }

    let response = query.get().await?;

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
    if !args.unicode {
        fixers::unicode(rec);
    }
}

async fn get(args: GetArgs, color: Color) -> Result<()> {
    let mut rec = dblp::Record::get(&args.key).await?;
    fixup(&mut rec, &args.common);
    let mut bibtex = rec.bibtex();
    if color.should_color(&std::io::stdout()) {
        bibtex.colorize();
    }
    println!("{bibtex}");
    Ok(())
}

async fn get_all(mut args: GetAllArgs, color: Color) -> Result<()> {
    args.path.set_extension("aux");
    let keys: Result<Vec<_>, _> =
        latex::CiteKeyIter::new(io::BufReader::new(fs::File::open(args.path)?)).collect();
    let mut keys = keys?;
    keys.sort_unstable();
    keys.dedup();
    let client = reqwest::Client::new();
    let n_keys = keys.len();
    for (idx, key) in keys.into_iter().enumerate() {
        if !key.starts_with("DBLP:") {
            continue;
        }
        let mut rec = dblp::Record::get_with_client(&key, client.clone()).await?;
        fixup(&mut rec, &args.common);
        let mut bibtex = rec.bibtex();
        if color.should_color(&std::io::stdout()) {
            bibtex.colorize();
        }
        println!("{bibtex}");
        if idx + 1 < n_keys {
            println!();
        }
    }
    Ok(())
}

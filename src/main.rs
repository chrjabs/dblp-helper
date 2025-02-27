use clap::Parser;
use cli::{Color, GetArgs, SearchArgs};
use color_eyre::eyre::Result;

mod cli;
mod dblp;
mod serde_utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = cli::Args::parse();
    args.color.init();

    match args.command {
        cli::Commands::Search(search_args) => search(search_args, args.color).await,
        cli::Commands::Get(get_args) => get(get_args, args.color).await,
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

async fn get(args: GetArgs, color: Color) -> Result<()> {
    let rec = dblp::Record::get(&args.key).await?;
    let mut bibtex = rec.bibtex();
    if color.should_color(&std::io::stdout()) {
        bibtex.colorize();
    }
    println!("{bibtex}");
    Ok(())
}

use clap::Parser;
use color_eyre::eyre::Result;

mod cli;
mod dblp;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = cli::Args::parse();
    args.color.init();
    let mut query = dblp::Query::new(args.r#type, &args.query);
    if let Some(hits) = args.hits {
        query.hits(hits);
    }

    let response = query.get().await?.json::<dblp::Response>().await?;

    for hit in response.iter_hits() {
        let mut display = hit.display();
        if args.color.should_color(&std::io::stdout()) {
            display.colorize();
        }
        println!();
        println!("{display}");
    }

    Ok(())
}

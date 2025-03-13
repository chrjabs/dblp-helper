pub mod record;
pub mod search;

pub use record::Record;

fn domain(opts: &super::cli::DblpServerArgs) -> &str {
    if let Some(domain) = &opts.dblp_domain {
        return domain;
    }
    if opts.trier {
        return "https://dblp.uni-trier.de";
    }
    "https://dblp.org"
}

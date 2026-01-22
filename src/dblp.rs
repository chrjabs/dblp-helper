pub mod record;
pub mod search;
mod stream;

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

pub fn new_service(
    opts: &super::cli::DblpServerArgs,
) -> impl tower::Service<
    reqwest::Request,
    Response = reqwest::Response,
    Error = Box<dyn std::error::Error + Send + std::marker::Sync + 'static>,
> + Clone {
    tower::ServiceBuilder::new()
        .buffer(opts.concurrent_requests)
        .rate_limit(1, std::time::Duration::from_millis(opts.rate_limit))
        .service(reqwest::Client::new())
}

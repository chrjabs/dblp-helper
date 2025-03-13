use std::fmt;

use reqwest::Url;

mod response;

pub use response::Response;

const PUBL_BASE: &str = "/search/publ/api";
const AUTHOR_BASE: &str = "/search/author/api";
const VENUE_BASE: &str = "/search/author/api";

#[derive(Clone, Copy, Debug, Default, clap::ValueEnum)]
pub enum Type {
    #[default]
    Publication,
    Author,
    Venue,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Publication => write!(f, "publication"),
            Type::Author => write!(f, "author"),
            Type::Venue => write!(f, "venue"),
        }
    }
}

impl Type {
    fn base_url(self) -> &'static str {
        match self {
            Type::Publication => PUBL_BASE,
            Type::Author => AUTHOR_BASE,
            Type::Venue => VENUE_BASE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Query {
    qtype: Type,
    query: String,
    hits: Option<u32>,
    first: Option<u32>,
    completions: Option<u32>,
}

impl Query {
    pub fn new<Q: Into<String>>(qtype: Type, query: Q) -> Self {
        Self {
            qtype,
            query: query.into(),
            hits: None,
            first: None,
            completions: None,
        }
    }

    pub fn hits(&mut self, val: u32) -> &mut Self {
        self.hits = Some(val);
        self
    }

    pub fn first(&mut self, val: u32) -> &mut Self {
        self.first = Some(val);
        self
    }

    pub fn completions(&mut self, val: u32) -> &mut Self {
        self.completions = Some(val);
        self
    }

    pub fn request_url(self, opts: &crate::cli::DblpServerArgs) -> Url {
        let mut url = Url::parse_with_params(
            &format!("{}{}", super::domain(opts), self.qtype.base_url()),
            [("q", self.query.as_str()), ("format", "json")],
        )
        .expect("base url must be valid");
        if let Some(hits) = self.hits {
            url.query_pairs_mut().append_pair("h", &hits.to_string());
        }
        if let Some(first) = self.first {
            url.query_pairs_mut().append_pair("f", &first.to_string());
        }
        if let Some(comp) = self.completions {
            url.query_pairs_mut().append_pair("c", &comp.to_string());
        }
        url
    }

    pub async fn get(self, opts: &crate::cli::DblpServerArgs) -> reqwest::Result<Response> {
        reqwest::get(self.request_url(opts))
            .await?
            .json::<Response>()
            .await
    }
}

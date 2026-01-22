//! # DBLP Streams
//!
//! Resolved from https://dblp.org/streams/

use std::fmt;

use tower::ServiceExt;

use super::record::Error;

const BASE: &str = "/streams/";

pub async fn journal_title<Service>(
    key: &str,
    opts: &crate::cli::DblpServerArgs,
    service: &mut Service,
) -> Result<String, Error>
where
    Service: tower::Service<
            reqwest::Request,
            Response = reqwest::Response,
            Error = Box<dyn std::error::Error + Send + std::marker::Sync + 'static>,
        >,
{
    let response = service
        .ready()
        .await?
        .call(reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse(&format!("{}{BASE}journals/{key}.xml", super::domain(opts)))
                .expect("this is a valid URL"),
        ))
        .await?;
    match response.status() {
        reqwest::StatusCode::NOT_FOUND => return Err(Error::UnknownKey(String::from(key))),
        code if !code.is_success() => return Err(Error::Http(code)),
        _ => {}
    }
    let Data::Journal { title, .. } =
        quick_xml::de::from_str::<XmlRecord>(&response.text().await?)?.value;
    Ok(format!("{title}"))
}

#[derive(Clone, Debug, serde::Deserialize)]
struct XmlRecord {
    #[serde(rename = "$value")]
    value: Data,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Data {
    Journal { title: Title },
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Title {
    #[serde(rename = "$value")]
    title: Vec<String>,
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for part in &self.title {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", part.trim())?;
            first = false;
        }
        Ok(())
    }
}

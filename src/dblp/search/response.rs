use std::fmt;

use owo_colors::OwoColorize;

use crate::cli::Styles;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Response {
    result: Result,
}

impl Response {
    pub fn iter_hits(&self) -> std::slice::Iter<Hit> {
        self.result.hits.hits.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Result {
    completions: Completions,
    hits: Hits,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Completions {
    #[serde(rename = "@total")]
    #[serde(deserialize_with = "crate::serde_utils::deserialize_number_from_string")]
    total: u32,
    #[serde(rename = "c", with = "crate::serde_utils::maybe_single")]
    completions: Vec<Completion>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Completion {
    text: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Hits {
    #[serde(
        rename = "@total",
        deserialize_with = "crate::serde_utils::deserialize_number_from_string"
    )]
    total: u32,
    #[serde(rename = "hit", with = "crate::serde_utils::maybe_single")]
    hits: Vec<Hit>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Hit {
    info: Info,
}

impl Hit {
    pub fn display(&self) -> HitDisplay<'_> {
        HitDisplay {
            value: self,
            styles: Box::default(),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct CommonInfo {
    authors: Authors,
    title: String,
    #[serde(deserialize_with = "crate::serde_utils::deserialize_number_from_string")]
    year: u32,
    access: Access,
    key: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
enum Access {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct CommonPaperInfo {
    venue: String,
    doi: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum Info {
    #[serde(rename = "Books and Theses")]
    BooksAndTheses {
        #[serde(flatten)]
        common: CommonInfo,
    },
    #[serde(rename = "Conference and Workshop Papers")]
    ConferenceAndWorkshopPapers {
        #[serde(flatten)]
        common: CommonInfo,
        #[serde(flatten)]
        paper: CommonPaperInfo,
    },
    #[serde(rename = "Journal Articles")]
    JournalArticles {
        #[serde(flatten)]
        common: CommonInfo,
        #[serde(flatten)]
        paper: CommonPaperInfo,
    },
    #[serde(rename = "Parts in Books or Collections")]
    PartsInBooksOrCollections {
        #[serde(flatten)]
        common: CommonInfo,
        venue: String,
        doi: String,
    },
    #[serde(rename = "Informal and Other Publications")]
    InformalAndOtherPublications {
        #[serde(flatten)]
        common: CommonInfo,
        venue: String,
        doi: String,
    },
    #[serde(rename = "Data and Artifacts")]
    DataAndArtifacts {
        #[serde(flatten)]
        common: CommonInfo,
    },
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Authors {
    #[serde(with = "crate::serde_utils::maybe_single")]
    author: Vec<Author>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Author {
    text: String,
}

/// Displayer for [`Hit`]
pub struct HitDisplay<'a> {
    value: &'a Hit,
    styles: Box<Styles>,
}

impl HitDisplay<'_> {
    /// Colorizes the output
    pub fn colorize(&mut self) {
        self.styles.colorize();
    }
}

impl fmt::Display for HitDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let common = match &self.value.info {
            Info::BooksAndTheses { common }
            | Info::ConferenceAndWorkshopPapers { common, .. }
            | Info::JournalArticles { common, .. }
            | Info::PartsInBooksOrCollections { common, .. }
            | Info::InformalAndOtherPublications { common, .. }
            | Info::DataAndArtifacts { common, .. } => common,
        };
        writeln!(f, "DBLP:{}", common.key.style(self.styles.citekey))?;
        writeln!(f, "{}", "----------".style(self.styles.separator))?;
        for (idx, author) in common.authors.author.iter().enumerate() {
            write!(f, "{}", author.text.style(self.styles.authors))?;
            if idx + 1 < common.authors.author.len() {
                write!(f, ", ")?;
            }
        }
        writeln!(f)?;
        writeln!(f, "{}", common.title.style(self.styles.title))?;
        match &self.value.info {
            Info::ConferenceAndWorkshopPapers {
                paper: CommonPaperInfo { venue, .. },
                ..
            }
            | Info::JournalArticles {
                paper: CommonPaperInfo { venue, .. },
                ..
            }
            | Info::PartsInBooksOrCollections { venue, .. }
            | Info::InformalAndOtherPublications { venue, .. } => {
                write!(f, "{}", venue.style(self.styles.venue))?;
                write!(f, ", ")?;
            }
            _ => {}
        };
        writeln!(f, "({})", common.year.style(self.styles.year))?;
        match &self.value.info {
            Info::ConferenceAndWorkshopPapers {
                paper: CommonPaperInfo { doi, .. },
                ..
            }
            | Info::JournalArticles {
                paper: CommonPaperInfo { doi, .. },
                ..
            }
            | Info::PartsInBooksOrCollections { doi, .. }
            | Info::InformalAndOtherPublications { doi, .. } => {
                write!(
                    f,
                    "{} ",
                    format!("https://doi.org/{doi}").style(self.styles.url)
                )?;
            }
            _ => {}
        };
        match common.access {
            Access::Open => writeln!(f, "[{}]", "open access".style(self.styles.open_access)),
            Access::Closed => writeln!(f, "[{}]", "closed access".style(self.styles.closed_access)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let data = r#"
            {
            "result":{
            "query":"test*",
            "status":{
            "@code":"200",
            "text":"OK"
            },
            "time":{
            "@unit":"msecs",
            "text":"2.93"
            },
            "completions":{
            "@total":"498",
            "@computed":"100",
            "@sent":"2",
            "c":[
            {
            "@sc":"48074",
            "@dc":"44805",
            "@oc":"48074",
            "@id":"59637089",
            "text":"test"
            },
            {
            "@sc":"41326",
            "@dc":"40495",
            "@oc":"41326",
            "@id":"59637375",
            "text":"testing"
            }
            ]
            },
            "hits":{
            "@total":"99047",
            "@computed":"100",
            "@sent":"1",
            "@first":"1",
            "hit":[{
            "@score":"5",
            "@id":"6425006",
            "info":{"authors":{"author":[{"@pid":"28/2391","text":"Tsuyoshi Shinogi"},{"@pid":"91/2238","text":"Hiroyuki Yamada"},{"@pid":"56/844","text":"Terumine Hayashi"},{"@pid":"21/5484","text":"Shinji Tsuruoka"},{"@pid":"67/2808","text":"Tomohiro Yoshikawa"}]},"title":"A Test Cost Reduction Method by Test Response and Test Vector Overlapping for Full-Scan Test Architecture.","venue":"Asian Test Symposium","pages":"366-371","year":"2005","type":"Conference and Workshop Papers","access":"closed","key":"conf/ats/ShinogiYHTY05","doi":"10.1109/ATS.2005.17","ee":"https://doi.org/10.1109/ATS.2005.17","url":"https://dblp.org/rec/conf/ats/ShinogiYHTY05"},
            "url":"URL#6425006"
            }
            ]
            }
            }
            }
        "#;

        let _: super::Response = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn single() {
        let data = r#"
            {
            "result":{
            "query":"test*",
            "status":{
            "@code":"200",
            "text":"OK"
            },
            "time":{
            "@unit":"msecs",
            "text":"2.93"
            },
            "completions":{
            "@total":"498",
            "@computed":"100",
            "@sent":"2",
            "c":[
            {
            "@sc":"48074",
            "@dc":"44805",
            "@oc":"48074",
            "@id":"59637089",
            "text":"test"
            },
            {
            "@sc":"41326",
            "@dc":"40495",
            "@oc":"41326",
            "@id":"59637375",
            "text":"testing"
            }
            ]
            },
            "hits":{
            "@total":"99047",
            "@computed":"100",
            "@sent":"1",
            "@first":"1",
            "hit":[{
            "@score":"5",
            "@id":"6425006",
            "info":{"authors":{"author":{"@pid":"28/2391","text":"Tsuyoshi Shinogi"}},"title":"A Test Cost Reduction Method by Test Response and Test Vector Overlapping for Full-Scan Test Architecture.","venue":"Asian Test Symposium","pages":"366-371","year":"2005","type":"Conference and Workshop Papers","access":"closed","key":"conf/ats/ShinogiYHTY05","doi":"10.1109/ATS.2005.17","ee":"https://doi.org/10.1109/ATS.2005.17","url":"https://dblp.org/rec/conf/ats/ShinogiYHTY05"},
            "url":"URL#6425006"
            }
            ]
            }
            }
            }
        "#;

        let _: super::Response = serde_json::from_str(data).unwrap();
    }
}

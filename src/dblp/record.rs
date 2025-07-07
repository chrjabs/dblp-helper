use std::fmt;

use owo_colors::OwoColorize;

use crate::cli::Styles;

const BASE: &str = "/rec/";

fn query_url(key: &str, opts: &crate::cli::DblpServerArgs) -> String {
    format!("{}{BASE}{key}.xml", super::domain(opts))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Xml(#[from] quick_xml::de::DeError),
    #[error("HTTP status code: {0}")]
    Http(reqwest::StatusCode),
    #[error("DBLP key `{0}` is unknown")]
    UnknownKey(String),
}

#[derive(Clone, Debug)]
pub enum Record {
    Article {
        key: String,
        author: Vec<String>,
        title: String,
        journal: String,
        year: u32,
        pages: Option<String>,
        volume: Option<String>,
        external: Vec<External>,
    },
    Proceedings {
        key: String,
        editor: Vec<String>,
        title: String,
        year: u32,
        series: Option<String>,
        volume: Option<String>,
        publisher: Option<String>,
        external: Vec<External>,
        isbn: Vec<String>,
    },
    Inproceedings {
        key: String,
        author: Vec<String>,
        title: String,
        booktitle: String,
        year: u32,
        pages: Option<String>,
        external: Vec<External>,
        crossref: Crossref,
    },
    Book {
        key: String,
        author: Vec<String>,
        editor: Vec<String>,
        title: String,
        publisher: Option<String>,
        year: u32,
        series: Option<String>,
        volume: Option<String>,
        external: Vec<External>,
        isbn: Vec<String>,
    },
    Incollection {
        key: String,
        author: Vec<String>,
        title: String,
        booktitle: String,
        year: u32,
        pages: Option<String>,
        external: Vec<External>,
        crossref: Crossref,
    },
}

#[derive(Clone, Debug)]
pub enum Crossref {
    Key(String),
    Resolved {
        editor: Vec<String>,
        publisher: Option<String>,
        series: Option<String>,
        volume: Option<String>,
    },
}

impl Record {
    pub async fn get(
        key: &str,
        resolve_crossref: bool,
        expand_journal: bool,
        opts: &crate::cli::DblpServerArgs,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::new();
        Self::get_with_client(key, resolve_crossref, expand_journal, opts, &client).await
    }

    pub async fn get_with_client(
        key: &str,
        resolve_crossref: bool,
        expand_journal: bool,
        opts: &crate::cli::DblpServerArgs,
        client: &reqwest::Client,
    ) -> Result<Self, Error> {
        let key = key.strip_prefix("DBLP:").unwrap_or(key);
        let response = client.get(query_url(key, opts)).send().await?;
        match response.status() {
            reqwest::StatusCode::NOT_FOUND => return Err(Error::UnknownKey(String::from(key))),
            code if !code.is_success() => return Err(Error::Http(code)),
            _ => {}
        }
        let rec = match quick_xml::de::from_str::<XmlRecord>(&response.text().await?)?.value {
            Data::Article {
                author,
                title,
                pages,
                year,
                volume,
                journal,
                ee,
            } => {
                if expand_journal {
                    let journal_key = key.split_once('/').unwrap().1.split_once('/').unwrap().0;
                    let journal = super::stream::journal_title(journal_key, opts, client).await?;
                    Self::Article {
                        key: key.to_string(),
                        author,
                        title,
                        pages,
                        year,
                        volume,
                        journal,
                        external: ee.into_iter().map(External::from).collect(),
                    }
                } else {
                    Self::Article {
                        key: key.to_string(),
                        author,
                        title,
                        pages,
                        year,
                        volume,
                        journal,
                        external: ee.into_iter().map(External::from).collect(),
                    }
                }
            }
            Data::Inproceedings {
                author,
                title,
                pages,
                year,
                booktitle,
                ee,
                crossref,
                ..
            } => {
                if resolve_crossref {
                    let response = client.get(query_url(&crossref, opts)).send().await?;
                    match response.status() {
                        reqwest::StatusCode::NOT_FOUND => {
                            panic!("this really shouldn't happen, or DBLP's data is buggy")
                        }
                        code if !code.is_success() => return Err(Error::Http(code)),
                        _ => {}
                    }
                    let Data::Proceedings {
                        editor,
                        title: booktitle,
                        series,
                        volume,
                        publisher,
                        ..
                    } = quick_xml::de::from_str::<XmlRecord>(&response.text().await?)?.value
                    else {
                        panic!("crossref data does not match");
                    };
                    Self::Inproceedings {
                        key: key.to_string(),
                        author,
                        title,
                        booktitle,
                        year,
                        pages,
                        external: ee.into_iter().map(External::from).collect(),
                        crossref: Crossref::Resolved {
                            editor,
                            publisher,
                            series,
                            volume,
                        },
                    }
                } else {
                    Self::Inproceedings {
                        key: key.to_string(),
                        author,
                        title,
                        booktitle,
                        year,
                        pages,
                        external: ee.into_iter().map(External::from).collect(),
                        crossref: Crossref::Key(crossref),
                    }
                }
            }
            Data::Incollection {
                author,
                title,
                pages,
                year,
                booktitle,
                ee,
                crossref,
                ..
            } => {
                if resolve_crossref {
                    let response = client.get(query_url(&crossref, opts)).send().await?;
                    match response.status() {
                        reqwest::StatusCode::NOT_FOUND => {
                            panic!("this really shouldn't happen, or DBLP's data is buggy")
                        }
                        code if !code.is_success() => return Err(Error::Http(code)),
                        _ => {}
                    }
                    let Data::Book {
                        editor,
                        title: booktitle,
                        series,
                        volume,
                        publisher,
                        ..
                    } = quick_xml::de::from_str::<XmlRecord>(&response.text().await?)?.value
                    else {
                        panic!("crossref data does not match");
                    };
                    Self::Incollection {
                        key: key.to_string(),
                        author,
                        title,
                        booktitle,
                        year,
                        pages,
                        external: ee.into_iter().map(External::from).collect(),
                        crossref: Crossref::Resolved {
                            editor,
                            publisher,
                            series,
                            volume,
                        },
                    }
                } else {
                    Self::Incollection {
                        key: key.to_string(),
                        author,
                        title,
                        booktitle,
                        year,
                        pages,
                        external: ee.into_iter().map(External::from).collect(),
                        crossref: Crossref::Key(crossref),
                    }
                }
            }
            Data::Proceedings {
                editor,
                title,
                publisher,
                year,
                series,
                volume,
                isbn,
                ee,
            } => Self::Proceedings {
                key: key.to_string(),
                editor,
                title,
                publisher,
                year,
                series,
                volume,
                isbn,
                external: ee.into_iter().map(External::from).collect(),
            },
            Data::Book {
                author,
                editor,
                title,
                publisher,
                year,
                series,
                volume,
                isbn,
                ee,
            } => Self::Book {
                key: key.to_string(),
                author,
                editor,
                title,
                publisher,
                year,
                series,
                volume,
                isbn,
                external: ee.into_iter().map(External::from).collect(),
            },
        };
        Ok(rec)
    }

    pub fn bibtex(&self) -> Bibtex<'_> {
        Bibtex {
            value: self,
            styles: Box::default(),
        }
    }

    pub fn crossref_key(&self) -> Option<&str> {
        if let Record::Inproceedings {
            crossref: Crossref::Key(key),
            ..
        }
        | Record::Incollection {
            crossref: Crossref::Key(key),
            ..
        } = self
        {
            return Some(key);
        }
        None
    }

    pub fn key(&self) -> &str {
        match self {
            Record::Article { key, .. }
            | Record::Proceedings { key, .. }
            | Record::Inproceedings { key, .. }
            | Record::Book { key, .. }
            | Record::Incollection { key, .. } => key,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Record::Article { title, .. }
            | Record::Proceedings { title, .. }
            | Record::Inproceedings { title, .. }
            | Record::Book { title, .. }
            | Record::Incollection { title, .. } => title,
        }
    }
}

#[derive(Clone, Debug)]
pub enum External {
    Url(String),
    Doi(String),
}

impl From<String> for External {
    fn from(mut value: String) -> Self {
        if value.starts_with("https://doi.org/") {
            value.drain(..16);
            return External::Doi(value);
        }
        External::Url(value)
    }
}

/// Bibtex displayer for [`Record`]
pub struct Bibtex<'a> {
    value: &'a Record,
    styles: Box<Styles>,
}

impl Bibtex<'_> {
    /// Colorizes the output
    pub fn colorize(&mut self) {
        self.styles.colorize();
    }
}

fn bibtex_start(
    f: &mut fmt::Formatter<'_>,
    bibtype: &str,
    key: &str,
    styles: &Styles,
) -> fmt::Result {
    writeln!(
        f,
        "@{bibtype}{{DBLP:{key},",
        bibtype = bibtype.style(styles.bibtex_type),
        key = key.style(styles.citekey)
    )
}

fn bibtex_people(
    f: &mut fmt::Formatter<'_>,
    key: &str,
    people: &[String],
    styles: &Styles,
) -> fmt::Result {
    if people.is_empty() {
        return Ok(());
    }
    write!(f, "  {key: <12} = {{", key = key.style(styles.bibtex_key))?;
    for (idx, peop) in people.iter().enumerate() {
        write!(f, "{peop}", peop = peop.style(styles.bibtex_val))?;
        if idx + 1 < people.len() {
            write!(f, " and ")?;
        }
    }
    writeln!(f, "}},")
}

fn bibtex_kv<V: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    key: &str,
    val: &V,
    styles: &Styles,
) -> fmt::Result {
    writeln!(
        f,
        "  {key: <12} = {{{val}}},",
        key = key.style(styles.bibtex_key),
        val = val.style(styles.bibtex_val),
    )
}

fn bibtex_end(f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "}}")
}

impl fmt::Display for Bibtex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Record::Article {
                key,
                author,
                title,
                pages,
                year,
                volume,
                journal,
                external,
            } => {
                bibtex_start(f, "article", key, &self.styles)?;
                bibtex_people(f, "author", author, &self.styles)?;
                bibtex_kv(f, "title", title, &self.styles)?;
                bibtex_kv(f, "journal", journal, &self.styles)?;
                bibtex_kv(f, "year", year, &self.styles)?;
                if let Some(pages) = pages {
                    bibtex_kv(f, "pages", pages, &self.styles)?;
                }
                if let Some(volume) = volume {
                    bibtex_kv(f, "volume", volume, &self.styles)?;
                }
                for external in external {
                    match external {
                        External::Url(url) => {
                            bibtex_kv(f, "url", url, &self.styles)?;
                        }
                        External::Doi(doi) => {
                            bibtex_kv(f, "doi", doi, &self.styles)?;
                        }
                    }
                }
                bibtex_end(f)
            }
            Record::Proceedings {
                key,
                editor,
                title,
                series,
                volume,
                publisher,
                year,
                external,
                isbn,
            } => {
                bibtex_start(f, "proceedings", key, &self.styles)?;
                bibtex_people(f, "editor", editor, &self.styles)?;
                bibtex_kv(f, "title", title, &self.styles)?;
                bibtex_kv(f, "year", year, &self.styles)?;
                if let Some(series) = series {
                    bibtex_kv(f, "series", series, &self.styles)?;
                }
                if let Some(volume) = volume {
                    bibtex_kv(f, "volume", volume, &self.styles)?;
                }
                if let Some(publisher) = publisher {
                    bibtex_kv(f, "publisher", publisher, &self.styles)?;
                }
                for isbn in isbn {
                    bibtex_kv(f, "isbn", isbn, &self.styles)?;
                }
                for external in external {
                    match external {
                        External::Url(url) => {
                            bibtex_kv(f, "url", url, &self.styles)?;
                        }
                        External::Doi(doi) => {
                            bibtex_kv(f, "doi", doi, &self.styles)?;
                        }
                    }
                }
                bibtex_end(f)
            }
            Record::Inproceedings {
                key,
                author,
                title,
                booktitle,
                pages,
                year,
                external,
                crossref,
            } => {
                bibtex_start(f, "inproceedings", key, &self.styles)?;
                bibtex_people(f, "author", author, &self.styles)?;
                bibtex_kv(f, "title", title, &self.styles)?;
                bibtex_kv(f, "booktitle", booktitle, &self.styles)?;
                bibtex_kv(f, "year", year, &self.styles)?;
                if let Some(pages) = pages {
                    bibtex_kv(f, "pages", pages, &self.styles)?;
                }
                for external in external {
                    match external {
                        External::Url(url) => {
                            bibtex_kv(f, "url", url, &self.styles)?;
                        }
                        External::Doi(doi) => {
                            bibtex_kv(f, "doi", doi, &self.styles)?;
                        }
                    }
                }
                match crossref {
                    Crossref::Key(key) => {
                        bibtex_kv(f, "crossref", &format!("DBLP:{key}"), &self.styles)?
                    }
                    Crossref::Resolved {
                        editor,
                        publisher,
                        series,
                        volume,
                    } => {
                        bibtex_people(f, "editor", editor, &self.styles)?;
                        if let Some(series) = series {
                            bibtex_kv(f, "series", series, &self.styles)?;
                        }
                        if let Some(volume) = volume {
                            bibtex_kv(f, "volume", volume, &self.styles)?;
                        }
                        if let Some(publisher) = publisher {
                            bibtex_kv(f, "publisher", publisher, &self.styles)?;
                        }
                    }
                }
                bibtex_end(f)
            }
            Record::Book {
                key,
                author,
                editor,
                title,
                year,
                volume,
                series,
                publisher,
                external,
                isbn,
            } => {
                bibtex_start(f, "book", key, &self.styles)?;
                bibtex_people(f, "author", author, &self.styles)?;
                bibtex_people(f, "editor", editor, &self.styles)?;
                bibtex_kv(f, "title", title, &self.styles)?;
                if let Some(publisher) = publisher {
                    bibtex_kv(f, "publisher", publisher, &self.styles)?;
                }
                bibtex_kv(f, "year", year, &self.styles)?;
                if let Some(series) = series {
                    bibtex_kv(f, "series", series, &self.styles)?;
                }
                if let Some(volume) = volume {
                    bibtex_kv(f, "volume", volume, &self.styles)?;
                }
                for isbn in isbn {
                    bibtex_kv(f, "isbn", isbn, &self.styles)?;
                }
                for external in external {
                    match external {
                        External::Url(url) => {
                            bibtex_kv(f, "url", url, &self.styles)?;
                        }
                        External::Doi(doi) => {
                            bibtex_kv(f, "doi", doi, &self.styles)?;
                        }
                    }
                }
                bibtex_end(f)
            }
            Record::Incollection {
                key,
                author,
                title,
                booktitle,
                pages,
                year,
                external,
                crossref,
            } => {
                bibtex_start(f, "incollection", key, &self.styles)?;
                bibtex_people(f, "author", author, &self.styles)?;
                bibtex_kv(f, "title", title, &self.styles)?;
                bibtex_kv(f, "booktitle", booktitle, &self.styles)?;
                bibtex_kv(f, "year", year, &self.styles)?;
                if let Some(pages) = pages {
                    bibtex_kv(f, "pages", pages, &self.styles)?;
                }
                for external in external {
                    match external {
                        External::Url(url) => {
                            bibtex_kv(f, "url", url, &self.styles)?;
                        }
                        External::Doi(doi) => {
                            bibtex_kv(f, "doi", doi, &self.styles)?;
                        }
                    }
                }
                match crossref {
                    Crossref::Key(key) => {
                        bibtex_kv(f, "crossref", &format!("DBLP:{key}"), &self.styles)?
                    }
                    Crossref::Resolved {
                        editor,
                        publisher,
                        series,
                        volume,
                    } => {
                        bibtex_people(f, "editor", editor, &self.styles)?;
                        if let Some(series) = series {
                            bibtex_kv(f, "series", series, &self.styles)?;
                        }
                        if let Some(volume) = volume {
                            bibtex_kv(f, "volume", volume, &self.styles)?;
                        }
                        if let Some(publisher) = publisher {
                            bibtex_kv(f, "publisher", publisher, &self.styles)?;
                        }
                    }
                }
                bibtex_end(f)
            }
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
struct XmlRecord {
    #[serde(rename = "$value")]
    value: Data,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Data {
    Article {
        author: Vec<String>,
        title: String,
        journal: String,
        year: u32,
        pages: Option<String>,
        volume: Option<String>,
        #[serde(default)]
        ee: Vec<String>,
    },
    Inproceedings {
        author: Vec<String>,
        title: String,
        pages: Option<String>,
        year: u32,
        booktitle: String,
        #[serde(default)]
        ee: Vec<String>,
        crossref: String,
    },
    Incollection {
        author: Vec<String>,
        title: String,
        year: u32,
        booktitle: String,
        pages: Option<String>,
        ee: Option<String>,
        crossref: String,
    },
    Proceedings {
        #[serde(default)]
        editor: Vec<String>,
        title: String,
        year: u32,
        series: Option<String>,
        volume: Option<String>,
        publisher: Option<String>,
        #[serde(default)]
        isbn: Vec<String>,
        #[serde(default)]
        ee: Vec<String>,
    },
    Book {
        #[serde(default)]
        author: Vec<String>,
        #[serde(default)]
        editor: Vec<String>,
        title: String,
        publisher: Option<String>,
        year: u32,
        series: Option<String>,
        volume: Option<String>,
        #[serde(default)]
        isbn: Vec<String>,
        #[serde(default)]
        ee: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn article() {
        let data = r#"
            <?xml version="1.0" encoding="US-ASCII"?>
            <dblp>
            <article key="journals/jair/JabsBNJ24" mdate="2024-10-06">
            <author>Christoph Jabs</author>
            <author orcid="0000-0001-7660-8061">Jeremias Berg</author>
            <author>Andreas Niskanen</author>
            <author>Matti J&#228;rvisalo</author>
            <title>From Single-Objective to Bi-Objective Maximum Satisfiability Solving.</title>
            <pages>1223-1269</pages>
            <year>2024</year>
            <volume>80</volume>
            <journal>J. Artif. Intell. Res.</journal>
            <ee type="oa">https://doi.org/10.1613/jair.1.15333</ee>
            <url>db/journals/jair/jair80.html#JabsBNJ24</url>
            <stream>streams/journals/jair</stream>
            </article>
            </dblp>
        "#;
        quick_xml::de::from_str::<super::XmlRecord>(data).unwrap();
    }

    #[test]
    fn inproceedings() {
        let data = r#"
            <?xml version="1.0" encoding="US-ASCII"?>
            <dblp>
            <inproceedings key="conf/cpaior/JabsBJ24" mdate="2024-07-04">
            <author orcid="0000-0003-3532-696X">Christoph Jabs</author>
            <author orcid="0000-0001-7660-8061">Jeremias Berg</author>
            <author orcid="0000-0003-2572-063X">Matti J&#228;rvisalo</author>
            <title>Core Boosting in SAT-Based Multi-objective Optimization.</title>
            <pages>1-19</pages>
            <year>2024</year>
            <booktitle>CPAIOR (2)</booktitle>
            <ee>https://doi.org/10.1007/978-3-031-60599-4_1</ee>
            <crossref>conf/cpaior/2024-2</crossref>
            <url>db/conf/cpaior/cpaior2024-2.html#JabsBJ24</url>
            </inproceedings>
            </dblp>
        "#;
        quick_xml::de::from_str::<super::XmlRecord>(data).unwrap();
    }

    #[test]
    fn proceedings() {
        let data = r#"
            <?xml version="1.0" encoding="US-ASCII"?>
            <dblp>
            <proceedings key="conf/cpaior/2024-2" mdate="2024-06-04">
            <editor orcid="0000-0002-6784-473X">Bistra Dilkina</editor>
            <title>Integration of Constraint Programming, Artificial Intelligence, and Operations Research - 21st International Conference, CPAIOR 2024, Uppsala, Sweden, May 28-31, 2024, Proceedings, Part II</title>
            <booktitle>CPAIOR (2)</booktitle>
            <publisher>Springer</publisher>
            <year>2024</year>
            <series href="db/series/lncs/index.html">Lecture Notes in Computer Science</series>
            <volume>14743</volume>
            <isbn>978-3-031-60601-4</isbn>
            <isbn>978-3-031-60599-4</isbn>
            <ee>https://doi.org/10.1007/978-3-031-60599-4</ee>
            <url>db/conf/cpaior/cpaior2024-2.html</url>
            </proceedings>
            </dblp>
        "#;
        quick_xml::de::from_str::<super::XmlRecord>(data).unwrap();
    }

    #[test]
    fn incollection() {
        let data = r#"
            <?xml version="1.0" encoding="US-ASCII"?>
            <dblp>
            <incollection key="series/faia/0001LM21" mdate="2023-09-30">
            <author>Jo&#227;o Marques-Silva 0001</author>
            <author>In&#234;s Lynce</author>
            <author orcid="0000-0002-0837-5443">Sharad Malik</author>
            <title>Conflict-Driven Clause Learning SAT Solvers.</title>
            <pages>133-182</pages>
            <year>2021</year>
            <booktitle>Handbook of Satisfiability</booktitle>
            <ee>https://doi.org/10.3233/FAIA200987</ee>
            <crossref>series/faia/336</crossref>
            <url>db/series/faia/faia336.html#0001LM21</url>
            </incollection></dblp>
        "#;
        quick_xml::de::from_str::<super::XmlRecord>(data).unwrap();
    }

    #[test]
    fn book() {
        let data = r#"
            <?xml version="1.0" encoding="US-ASCII"?>
            <dblp>
            <book key="series/faia/336" mdate="2022-05-06">
            <editor>Armin Biere</editor>
            <editor>Marijn Heule</editor>
            <editor>Hans van Maaren</editor>
            <editor>Toby Walsh</editor>
            <title>Handbook of Satisfiability - Second Edition</title>
            <publisher>IOS Press</publisher>
            <year>2021</year>
            <series href="db/series/faia/index.html">Frontiers in Artificial Intelligence and Applications</series>
            <volume>336</volume>
            <isbn>978-1-64368-160-3</isbn>
            <isbn>978-1-64368-161-0</isbn>
            <ee>https://doi.org/10.3233/FAIA336</ee>
            <url>db/series/faia/faia336.html</url>
            </book>
            </dblp>
        "#;
        quick_xml::de::from_str::<super::XmlRecord>(data).unwrap();
    }
}

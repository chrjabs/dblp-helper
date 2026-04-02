use biblatex::Type;

use crate::dblp::{
    Record,
    record::{Crossref, External},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid BibTeX Entry: {0}")]
    Parsing(#[from] biblatex::ParseError),
    #[error("Missing field {0}, for entry {1}")]
    MissingField(String, String),
}

pub fn parse(content: &str) -> Result<Vec<Record>, Error> {
    let mut records = vec![];
    for entry in biblatex::Bibliography::parse(&content)? {
        if !entry.key.starts_with("DBLP:") {
            eprintln!("Dropping non-DBLP entry in BibTeX file");
            eprintln!("{}", entry.to_biblatex_string());
            eprintln!("If this is important, move it to a separate BibTeX file");
        }
        let key = String::from(&entry.key[5..]);
        let Ok(title) = entry.title() else {
            return Err(Error::MissingField(String::from("title"), entry.key));
        };
        let title = String::from_chunks(title).unwrap();
        match entry.entry_type {
            biblatex::EntryType::Article => {
                let Ok(author) = entry.author() else {
                    return Err(Error::MissingField(String::from("author"), entry.key));
                };
                let Ok(journal) = entry.journal() else {
                    return Err(Error::MissingField(String::from("journal"), entry.key));
                };
                let journal = String::from_chunks(journal).unwrap();
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let pages = entry.pages().ok().map(format_pages);
                let volume = entry.volume().ok().map(format_volume);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                records.push(Record::Article {
                    key,
                    author: author.into_iter().map(format_person).collect(),
                    title,
                    journal,
                    year,
                    pages,
                    volume,
                    external,
                })
            }
            biblatex::EntryType::Proceedings => {
                let editors = entry.editors().unwrap_or(vec![]);
                let editor = editors.into_iter().next().map(|(p, _)| p).unwrap_or(vec![]);
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let series = entry
                    .series()
                    .ok()
                    .map(|val| String::from_chunks(val).unwrap());
                let volume = entry.volume().ok().map(format_volume);
                let publisher = entry.publisher().ok().map(format_publisher);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                let isbn = entry
                    .isbn()
                    .ok()
                    .map(|val| String::from_chunks(val).unwrap());
                records.push(Record::Proceedings {
                    key,
                    editor: editor.into_iter().map(format_person).collect(),
                    title,
                    year,
                    series,
                    volume,
                    publisher,
                    external,
                    isbn: isbn.into_iter().collect(),
                })
            }
            biblatex::EntryType::InProceedings => {
                let Ok(author) = entry.author() else {
                    return Err(Error::MissingField(String::from("author"), entry.key));
                };
                let Ok(booktitle) = entry.book_title() else {
                    return Err(Error::MissingField(String::from("booktitle"), entry.key));
                };
                let booktitle = String::from_chunks(booktitle).unwrap();
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let pages = entry.pages().ok().map(format_pages);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                let crossref = match entry.get("crossref") {
                    Some(crossref) => {
                        let key = String::from_chunks(crossref).unwrap();
                        let key = key.strip_prefix("DBLP:").unwrap_or(&key);
                        Crossref::Key(String::from(key))
                    }
                    None => {
                        let Ok(editors) = entry.editors() else {
                            return Err(Error::MissingField(String::from("editor"), entry.key));
                        };
                        let editor = editors.into_iter().next().map(|(p, _)| p).unwrap_or(vec![]);
                        let publisher = entry.publisher().ok().map(format_publisher);
                        let series = entry
                            .series()
                            .ok()
                            .map(|val| String::from_chunks(val).unwrap());
                        let volume = entry.volume().ok().map(format_volume);
                        Crossref::Resolved {
                            editor: editor.into_iter().map(format_person).collect(),
                            publisher,
                            series,
                            volume,
                        }
                    }
                };
                let usera = entry
                    .get("usera")
                    .map(|val| String::from_chunks(val).unwrap());
                records.push(Record::Inproceedings {
                    key,
                    author: author.into_iter().map(format_person).collect(),
                    title,
                    booktitle,
                    year,
                    pages,
                    external,
                    crossref,
                    usera,
                })
            }
            biblatex::EntryType::Book => {
                let author = entry.author().unwrap_or(vec![]);
                let editors = entry.editors().unwrap_or(vec![]);
                let editor = editors.into_iter().next().map(|(p, _)| p).unwrap_or(vec![]);
                let publisher = entry.publisher().ok().map(format_publisher);
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let series = entry
                    .series()
                    .ok()
                    .map(|val| String::from_chunks(val).unwrap());
                let volume = entry.volume().ok().map(format_volume);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                let isbn = entry
                    .isbn()
                    .ok()
                    .map(|val| String::from_chunks(val).unwrap());
                records.push(Record::Book {
                    key,
                    author: author.into_iter().map(format_person).collect(),
                    editor: editor.into_iter().map(format_person).collect(),
                    title,
                    publisher,
                    year,
                    series,
                    volume,
                    external,
                    isbn: isbn.into_iter().collect(),
                })
            }
            biblatex::EntryType::InCollection => {
                let Ok(author) = entry.author() else {
                    return Err(Error::MissingField(String::from("author"), entry.key));
                };
                let Ok(booktitle) = entry.book_title() else {
                    return Err(Error::MissingField(String::from("booktitle"), entry.key));
                };
                let booktitle = String::from_chunks(booktitle).unwrap();
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let pages = entry.pages().ok().map(format_pages);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                let crossref = match entry.get("crossref") {
                    Some(crossref) => {
                        let key = String::from_chunks(crossref).unwrap();
                        let key = key.strip_prefix("DBLP:").unwrap_or(&key);
                        Crossref::Key(String::from(key))
                    }
                    None => {
                        let editors = entry.editors().unwrap_or(vec![]);
                        let editor = editors.into_iter().next().map(|(p, _)| p).unwrap_or(vec![]);
                        let publisher = entry.publisher().ok().map(format_publisher);
                        let series = entry
                            .series()
                            .ok()
                            .map(|val| String::from_chunks(val).unwrap());
                        let volume = entry.volume().ok().map(format_volume);
                        Crossref::Resolved {
                            editor: editor.into_iter().map(format_person).collect(),
                            publisher,
                            series,
                            volume,
                        }
                    }
                };
                records.push(Record::Incollection {
                    key,
                    author: author.into_iter().map(format_person).collect(),
                    title,
                    booktitle,
                    year,
                    pages,
                    external,
                    crossref,
                })
            }
            biblatex::EntryType::Misc => {
                let Ok(author) = entry.author() else {
                    return Err(Error::MissingField(String::from("author"), entry.key));
                };
                let Ok(date) = entry.date() else {
                    return Err(Error::MissingField(String::from("year"), entry.key));
                };
                let year = format_year(date);
                let publisher = entry.publisher().ok().map(format_publisher);
                let mut external = vec![];
                if let Ok(doi) = entry.doi() {
                    external.push(External::Doi(doi));
                }
                if let Ok(url) = entry.url() {
                    external.push(External::Url(url));
                }
                records.push(Record::Misc {
                    key,
                    author: author.into_iter().map(format_person).collect(),
                    title,
                    year,
                    publisher,
                    external,
                })
            }
            _ => {
                eprintln!("Dropping unsupported entry in BibTeX file");
                eprintln!("{}", entry.to_biblatex_string());
                eprintln!("If this is important, move it to a separate BibTeX file");
            }
        }
    }
    Ok(records)
}

fn format_person(person: biblatex::Person) -> String {
    if person.suffix.is_empty() {
        if person.prefix.is_empty() {
            format!("{} {}", person.given_name, person.name)
        } else {
            format!("{} {} {}", person.given_name, person.prefix, person.name)
        }
    } else {
        format!(
            "{} {} {} {}",
            person.given_name, person.prefix, person.name, person.suffix,
        )
    }
}

fn format_year(date: biblatex::PermissiveType<biblatex::Date>) -> u32 {
    match date {
        biblatex::PermissiveType::Typed(date) => match date.value {
            biblatex::DateValue::At(datetime)
            | biblatex::DateValue::After(datetime)
            | biblatex::DateValue::Before(datetime)
            | biblatex::DateValue::Between(datetime, _) => u32::try_from(datetime.year).unwrap(),
        },
        biblatex::PermissiveType::Chunks(spanned) => {
            u32::try_from(i64::from_chunks(&spanned).unwrap()).unwrap()
        }
    }
}

fn format_pages(pages: biblatex::PermissiveType<Vec<std::ops::Range<u32>>>) -> String {
    match pages {
        biblatex::PermissiveType::Typed(ranges) => {
            assert_eq!(ranges.len(), 1);
            let range = ranges.into_iter().next().unwrap();
            format!("{}--{}", range.start, range.end)
        }
        biblatex::PermissiveType::Chunks(spanned) => String::from_chunks(&spanned).unwrap(),
    }
}

fn format_volume(volume: biblatex::PermissiveType<i64>) -> String {
    match volume {
        biblatex::PermissiveType::Typed(volume) => format!("{volume}"),
        biblatex::PermissiveType::Chunks(spanned) => String::from_chunks(&spanned).unwrap(),
    }
}

fn format_publisher(publisher: Vec<biblatex::Chunks>) -> String {
    assert_eq!(publisher.len(), 1);
    let publisher = publisher.into_iter().next().unwrap();
    String::from_chunks(&publisher).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn roundtrips() {
        let args = crate::cli::CommonGetArgs {
            unicode: false,
            crossref: true,
            all_externals: false,
            dont_expand_journals: false,
        };

        let orig = r#"@article{DBLP:journals/jair/JabsBNJ24,
  author       = {Jabs, Christoph and Berg, Jeremias and Niskanen, Andreas and J{\"a}rvisalo, Matti},
  title        = {From Single-Objective to Bi-Objective Maximum Satisfiability Solving},
  journal      = {Journal of Artificial Intelligence Research},
  year         = {2024},
  pages        = {1223--1269},
  volume       = {80},
  doi          = {10.1613/jair.1.15333},
}"#;
        let mut parsed = super::parse(orig).unwrap();
        super::super::fixup(&mut parsed[0], &args);
        assert_eq!(&format!("{}", parsed[0].bibtex()), orig);

        let orig = r#"@proceedings{DBLP:conf/cpaior/2024-2,
  editor       = {Dilkina, Bistra},
  title        = {Integration of Constraint Programming, Artificial Intelligence, and Operations Research{\textemdash}21st International Conference, {CPAIOR} 2024, Uppsala, Sweden, May 28{\textendash}31, 2024, Proceedings, Part {II}},
  year         = {2024},
  series       = {Lecture Notes in Computer Science},
  volume       = {14743},
  publisher    = {Springer},
  isbn         = {978-3-031-60601-4},
  doi          = {10.1007/978-3-031-60599-4},
}"#;
        let mut parsed = super::parse(orig).unwrap();
        super::super::fixup(&mut parsed[0], &args);
        assert_eq!(&format!("{}", parsed[0].bibtex()), orig);

        let orig = r#"@inproceedings{DBLP:conf/cp/JabsBIJ23,
  author       = {Jabs, Christoph and Berg, Jeremias and Ihalainen, Hannes and J{\"a}rvisalo, Matti},
  title        = {Preprocessing in {SAT}-Based Multi-Objective Combinatorial Optimization},
  booktitle    = {29th International Conference on Principles and Practice of Constraint Programming, {CP} 2023, Toronto, Canada, August 27{\textendash}31, 2023},
  year         = {2023},
  pages        = {18:1--18:20},
  doi          = {10.4230/LIPIcs.CP.2023.18},
  editor       = {Yap, Roland H. C.},
  series       = {LIPIcs},
  volume       = {280},
  publisher    = {Schloss Dagstuhl - Leibniz-Zentrum f{\"u}r Informatik},
  usera        = {Proceedings of International Conference on Principles and Practice of Constraint Programming, {CP} 2023},
}"#;
        let mut parsed = super::parse(orig).unwrap();
        super::super::fixup(&mut parsed[0], &args);
        assert_eq!(&format!("{}", parsed[0].bibtex()), orig);

        let orig = r#"@inproceedings{DBLP:conf/jelia/JabsBJ25,
  author       = {Jabs, Christoph and Berg, Jeremias and J{\"a}rvisalo, Matti},
  title        = {Engineering and Evaluating Multi-objective Pseudo-{Boolean} Optimizers},
  booktitle    = {Logics in Artificial Intelligence{\textemdash}19th European Conference, {JELIA} 2025, Kutaisi, Georgia, September 1{\textendash}4, 2025, Proceedings, Part I},
  year         = {2025},
  pages        = {115--134},
  doi          = {10.1007/978-3-032-04587-4_8},
  crossref     = {DBLP:conf/jelia/2025-1},
  usera        = {Logics in Artificial Intelligence, {JELIA} 2025},
}"#;
        let mut parsed = super::parse(orig).unwrap();
        super::super::fixup(&mut parsed[0], &args);
        assert_eq!(&format!("{}", parsed[0].bibtex()), orig);
    }
}

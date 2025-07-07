use lazy_static::lazy_static;
use regex::Regex;

use crate::dblp::{
    Record,
    record::{Crossref, External},
};

mod names;
mod unicode;

lazy_static! {
    static ref RANGE_PATTERN: Regex = Regex::new(r"(\d)-(\d)").unwrap();
    static ref AUTHOR_NUM_PATTERN: Regex = Regex::new(r" \d\d\d\d$").unwrap();
    static ref WORD_PATTERN: Regex = Regex::new(r"\w+").unwrap();
    static ref DATE_RANGE_PATTERN: Regex = Regex::new(r"(\d)-(\d)|(\d\s)-(\sJanuary|\sFebruary|\sMarch|\sApril|\sMay|\sJune|\sJuly|\sAugust|\sSeptember|\sOctober|\sNovember|\sDecember)").unwrap();
}

pub fn page_range(rec: &mut Record) {
    match rec {
        Record::Article { pages, .. }
        | Record::Inproceedings { pages, .. }
        | Record::Incollection { pages, .. } => {
            if let Some(pages) = pages {
                let rep = RANGE_PATTERN.replace(pages, "${1}--${2}");
                *pages = rep.to_string();
            }
        }
        _ => {}
    }
}

pub fn author_num(rec: &mut Record) {
    match rec {
        Record::Article { author, .. }
        | Record::Inproceedings { author, .. }
        | Record::Book { author, .. }
        | Record::Incollection { author, .. } => {
            for author in author.iter_mut() {
                let rep = AUTHOR_NUM_PATTERN.replace(author, "");
                *author = rep.to_string();
            }
        }
        _ => {}
    }
    match rec {
        Record::Proceedings { editor, .. }
        | Record::Inproceedings {
            crossref: Crossref::Resolved { editor, .. },
            ..
        }
        | Record::Book { editor, .. }
        | Record::Incollection {
            crossref: Crossref::Resolved { editor, .. },
            ..
        } => {
            for editor in editor.iter_mut() {
                let rep = AUTHOR_NUM_PATTERN.replace(editor, "");
                *editor = rep.to_string();
            }
        }
        _ => {}
    }
}

pub fn unicode(rec: &mut Record) {
    match rec {
        Record::Article {
            author,
            title,
            journal,
            ..
        } => {
            for author in author.iter_mut() {
                unicode::replace(author);
            }
            unicode::replace(title);
            unicode::replace(journal);
        }
        Record::Proceedings {
            editor,
            title,
            series,
            publisher,
            ..
        } => {
            for editor in editor.iter_mut() {
                unicode::replace(editor);
            }
            unicode::replace(title);
            if let Some(series) = series {
                unicode::replace(series);
            }
            if let Some(publisher) = publisher {
                unicode::replace(publisher);
            }
        }
        Record::Inproceedings {
            author,
            title,
            booktitle,
            crossref,
            ..
        } => {
            for author in author.iter_mut() {
                unicode::replace(author);
            }
            unicode::replace(title);
            unicode::replace(booktitle);
            if let Crossref::Resolved {
                editor,
                series,
                publisher,
                ..
            } = crossref
            {
                for editor in editor.iter_mut() {
                    unicode::replace(editor);
                }
                if let Some(series) = series {
                    unicode::replace(series);
                }
                if let Some(publisher) = publisher {
                    unicode::replace(publisher);
                }
            }
        }
        Record::Incollection {
            author,
            title,
            booktitle,
            crossref,
            ..
        } => {
            for author in author.iter_mut() {
                unicode::replace(author);
            }
            unicode::replace(title);
            unicode::replace(booktitle);
            if let Crossref::Resolved {
                editor,
                series,
                publisher,
                ..
            } = crossref
            {
                for editor in editor.iter_mut() {
                    unicode::replace(editor);
                }
                if let Some(series) = series {
                    unicode::replace(series);
                }
                if let Some(publisher) = publisher {
                    unicode::replace(publisher);
                }
            }
        }
        Record::Book {
            author,
            editor,
            title,
            series,
            publisher,
            ..
        } => {
            for author in author.iter_mut() {
                unicode::replace(author);
            }
            for editor in editor.iter_mut() {
                unicode::replace(editor);
            }
            unicode::replace(title);
            if let Some(series) = series {
                unicode::replace(series);
            }
            if let Some(publisher) = publisher {
                unicode::replace(publisher);
            }
        }
    }
}

pub fn names(rec: &mut Record) {
    match rec {
        Record::Article { author, .. }
        | Record::Inproceedings { author, .. }
        | Record::Book { author, .. }
        | Record::Incollection { author, .. } => {
            for author in author {
                names::fix(author);
            }
        }
        _ => {}
    }
    match rec {
        Record::Proceedings { editor, .. }
        | Record::Inproceedings {
            crossref: Crossref::Resolved { editor, .. },
            ..
        }
        | Record::Book { editor, .. }
        | Record::Incollection {
            crossref: Crossref::Resolved { editor, .. },
            ..
        } => {
            for editor in editor {
                names::fix(editor);
            }
        }
        _ => {}
    }
}

/// Wraps acronyms such as `SAT` of `MaxSAT` in curly braces
fn fix_acronyms(string: &mut String) {
    let mut changed = None;
    let mut offset = 0;
    for matched in WORD_PATTERN.find_iter(string) {
        // Acronym cases:
        // 1. has more than one upper case
        // 2. starts with lower case, but contains upper case
        let first_upper = matched
            .as_str()
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or(false);
        let n_upper = matched
            .as_str()
            .chars()
            .fold(0, |cnt, ch| if ch.is_uppercase() { cnt + 1 } else { cnt });
        if n_upper > 1 || (!first_upper && n_upper > 0) {
            if changed.is_none() {
                changed = Some(string.clone());
            }
            let changed = changed.as_mut().unwrap();
            // wrap in braces
            changed.insert(matched.end() + offset, '}');
            changed.insert(matched.start() + offset, '{');
            offset += 2;
        }
    }
    if let Some(changed) = changed {
        *string = changed;
    }
}

pub fn acronyms(rec: &mut Record) {
    let (Record::Article { title, .. }
    | Record::Proceedings { title, .. }
    | Record::Inproceedings { title, .. }
    | Record::Book { title, .. }
    | Record::Incollection { title, .. }) = rec;
    fix_acronyms(title);
    if let Record::Inproceedings { booktitle, .. } | Record::Incollection { booktitle, .. } = rec {
        fix_acronyms(booktitle);
    }
}

pub fn weird_urls(rec: &mut Record) {
    let (Record::Article { external, .. }
    | Record::Proceedings { external, .. }
    | Record::Inproceedings { external, .. }
    | Record::Book { external, .. }
    | Record::Incollection { external, .. }) = rec;
    external.retain(|ext| {
        !matches!(
            ext,
            External::Url(s) if s.starts_with("https://www.wikidata.org") || s.starts_with("https://ojs.aaai.org")
        )
    })
}

pub fn date_ranges(rec: &mut Record) {
    if let Record::Inproceedings { booktitle, .. }
    | Record::Proceedings {
        title: booktitle, ..
    } = rec
    {
        let rep = DATE_RANGE_PATTERN.replace(booktitle, "${1}${3}--${2}${4}");
        *booktitle = rep.to_string();
    }
}

pub fn dashes(rec: &mut Record) {
    let (Record::Article { title, .. }
    | Record::Proceedings { title, .. }
    | Record::Inproceedings { title, .. }
    | Record::Book { title, .. }
    | Record::Incollection { title, .. }) = rec;
    *title = title.replace(" - ", "---");
    if let Record::Article { journal: venue, .. }
    | Record::Inproceedings {
        booktitle: venue, ..
    }
    | Record::Incollection {
        booktitle: venue, ..
    } = rec
    {
        *venue = venue.replace(" - ", "---");
    }
}

pub fn expand_booktitle(rec: &mut Record, crossref: &Record) {
    match rec {
        Record::Inproceedings {
            booktitle,
            crossref: Crossref::Key(_),
            ..
        }
        | Record::Incollection {
            booktitle,
            crossref: Crossref::Key(_),
            ..
        } => {
            *booktitle = crossref.title().to_owned();
        }
        _ => panic!("got non-crossref record"),
    }
}

/// Manual fixer for mistakes in DBLP data
pub fn manually_correct(rec: &mut Record) {
    // Mistake introduced in the metadata in the final editing process
    if rec.key() == "conf/tacas/JabsBBJ25" {
        if let Record::Inproceedings { title, .. } = rec {
            *title = String::from(
                "Certifying Pareto Optimality in Multi-objective Maximum Satisfiability",
            );
        }
    }
}

/// Removes all but one external link
///
/// If a DOI is present, the first one is used, otherwise the first URL is used
pub fn single_external(rec: &mut Record) {
    match rec {
        Record::Article { external, .. }
        | Record::Proceedings { external, .. }
        | Record::Inproceedings { external, .. }
        | Record::Book { external, .. }
        | Record::Incollection { external, .. } => {
            let Some(mut chosen) = external.first().cloned() else {
                return;
            };
            for ext in external.iter() {
                if matches!(ext, External::Doi(_)) {
                    chosen = ext.clone();
                    break;
                }
            }
            *external = vec![chosen];
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn acronyms() {
        let mut text = String::from("SAT is an Acronym");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "{SAT} is an Acronym");

        let mut text = String::from("Another Acronym is MaxSAT");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "Another Acronym is {MaxSAT}");

        let mut text = String::from("With SAT and MaxSAT we have two acronyms");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "With {SAT} and {MaxSAT} we have two acronyms");

        let mut text = String::from("MaxSAT-based bi-objective optimization");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "{MaxSAT}-based bi-objective optimization");

        let mut text =
            String::from("Using Small MUSes to Explain How to Solve Pen and Paper Puzzles.");
        super::fix_acronyms(&mut text);
        assert_eq!(
            text,
            "Using Small {MUSes} to Explain How to Solve Pen and Paper Puzzles."
        );
    }
}

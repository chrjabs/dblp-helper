use itertools::Itertools;
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
    static ref WORD_PATTERN: Regex = Regex::new(r"[\w-]+").unwrap();
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

fn all_strings(rec: &mut Record, apply: impl Fn(&mut String)) {
    match rec {
        Record::Article {
            author,
            title,
            journal,
            ..
        } => {
            for author in author.iter_mut() {
                apply(author);
            }
            apply(title);
            apply(journal);
        }
        Record::Proceedings {
            editor,
            title,
            series,
            publisher,
            ..
        } => {
            for editor in editor.iter_mut() {
                apply(editor);
            }
            apply(title);
            if let Some(series) = series {
                apply(series);
            }
            if let Some(publisher) = publisher {
                apply(publisher);
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
                apply(author);
            }
            apply(title);
            apply(booktitle);
            if let Crossref::Resolved {
                editor,
                series,
                publisher,
                ..
            } = crossref
            {
                for editor in editor.iter_mut() {
                    apply(editor);
                }
                if let Some(series) = series {
                    apply(series);
                }
                if let Some(publisher) = publisher {
                    apply(publisher);
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
                apply(author);
            }
            apply(title);
            apply(booktitle);
            if let Crossref::Resolved {
                editor,
                series,
                publisher,
                ..
            } = crossref
            {
                for editor in editor.iter_mut() {
                    apply(editor);
                }
                if let Some(series) = series {
                    apply(series);
                }
                if let Some(publisher) = publisher {
                    apply(publisher);
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
                apply(author);
            }
            for editor in editor.iter_mut() {
                apply(editor);
            }
            apply(title);
            if let Some(series) = series {
                apply(series);
            }
            if let Some(publisher) = publisher {
                apply(publisher);
            }
        }
        Record::Misc {
            author,
            title,
            publisher,
            ..
        } => {
            for author in author.iter_mut() {
                apply(author);
            }
            apply(title);
            if let Some(publisher) = publisher {
                apply(publisher);
            }
        }
    }
}

fn escape_latex_chars(input: &mut String) {
    let mut out: Option<String> = None;
    for (idx, char) in input.chars().enumerate() {
        let repl = match char {
            '#' => Some(r"\#"),
            '$' => Some(r"\$"),
            '%' => Some(r"\%"),
            '&' => Some(r"\&"),
            '<' => Some(r"\ensuremath{<}"),
            '>' => Some(r"\ensuremath{>}"),
            '\\' => Some(r"\textbackslash{}"),
            '^' => Some(r"\textasciicircum{}"),
            '_' => Some(r"\_"),
            '{' => Some(r"\{"),
            '}' => Some(r"\}"),
            '~' => Some(r"\textasciitilde{}"),
            _ => None,
        };
        let Some(repl) = repl else {
            if let Some(out) = out.as_mut() {
                out.push(char);
            }
            continue;
        };
        if out.is_none() {
            out = Some(String::from(&input[..idx]));
        }
        let out = out.as_mut().unwrap();
        out.push_str(repl);
    }
    if let Some(out) = out {
        let _ = std::mem::replace(input, out);
    }
}

pub fn escape_latex(rec: &mut Record) {
    all_strings(rec, escape_latex_chars)
}

pub fn unicode(rec: &mut Record) {
    all_strings(rec, unicode::replace)
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
        // 1. has more than one upper case and not all of them are after a dash
        // 2. starts with lower case, but contains upper case
        let first_upper = matched
            .as_str()
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or(false);
        let (n_upper, n_upper_after_dash) = matched.as_str().chars().tuple_windows().fold(
            (if first_upper { 1 } else { 0 }, 0),
            |(total, after_dash), (first, second)| {
                (
                    if second.is_uppercase() {
                        total + 1
                    } else {
                        total
                    },
                    if first == '-' {
                        after_dash + 1
                    } else {
                        after_dash
                    },
                )
            },
        );
        let start = matched.start() + offset;
        let mut end = matched.end() + offset;
        // Special exceptions:
        // - `anything-based` (e.g., SAT-based), the `-based` will not be included in the acronym
        if matches!(
            matched.as_str().rsplit_once('-'),
            Some((_, "based")) | Some((_, "Based"))
        ) {
            end -= 6;
        }
        if n_upper - n_upper_after_dash > 1 || (!first_upper && n_upper > 0) {
            let changed = changed.get_or_insert_with(|| string.clone());
            // wrap in braces
            changed.insert(end, '}');
            changed.insert(start, '{');
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
    | Record::Incollection { title, .. }
    | Record::Misc { title, .. }) = rec;
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
    | Record::Incollection { external, .. }
    | Record::Misc { external, .. }) = rec;
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
    | Record::Incollection { title, .. }
    | Record::Misc { title, .. }) = rec;
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
        | Record::Incollection { external, .. }
        | Record::Misc { external, .. } => {
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

        let mut text = String::from("Some people write Max-SAT");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "Some people write {Max-SAT}");

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

        let mut text = String::from("Thirty-First should not be an ancronym");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "Thirty-First should not be an ancronym");

        let mut text = String::from("big-M should be an acronym");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "{big-M} should be an acronym");

        let mut text = String::from("SAT-Based and MaxSAT-Based are special exceptions");
        super::fix_acronyms(&mut text);
        assert_eq!(
            text,
            "{SAT}-Based and {MaxSAT}-Based are special exceptions"
        );
    }
}

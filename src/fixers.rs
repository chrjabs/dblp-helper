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
    static ref HYPHENATED_WORD_PATTERN: Regex = Regex::new(r"[\w\d-]+").unwrap();
    static ref DATE_RANGE_PATTERN: Regex = Regex::new(r"(\d)-(\d)|(\d\s)-(\sJanuary|\sFebruary|\sMarch|\sApril|\sMay|\sJune|\sJuly|\sAugust|\sSeptember|\sOctober|\sNovember|\sDecember)").unwrap();
    static ref PROPER_NOUNS_PATTERN: Regex = Regex::new(r"(Bool(ean)?|Pareto|Slide\\\&Drill|Seesaw)").unwrap();
    static ref CAPITAL_AFTER_COLON: Regex = Regex::new(r":\s+[A-Z]").unwrap();
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
    all_strings(rec, unicode::replace);
    match rec {
        Record::Article {
            pages: Some(pages), ..
        }
        | Record::Inproceedings {
            pages: Some(pages), ..
        }
        | Record::Incollection {
            pages: Some(pages), ..
        } => {
            *pages = pages.replace('–', "--");
        }
        _ => (),
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

fn fix_proper_nouns(string: &mut String) {
    let mut changed = None;
    let mut offset = 0;
    for matched in PROPER_NOUNS_PATTERN.find_iter(string) {
        let start = matched.start() + offset;
        let end = matched.end() + offset;
        let changed = changed.get_or_insert_with(|| string.clone());
        // wrap in braces
        changed.insert(end, '}');
        changed.insert(start, '{');
        offset += 2;
    }
    if let Some(changed) = changed {
        *string = changed;
    }
}

pub fn proper_nouns(rec: &mut Record) {
    let (Record::Article { title, .. }
    | Record::Proceedings { title, .. }
    | Record::Inproceedings { title, .. }
    | Record::Book { title, .. }
    | Record::Incollection { title, .. }
    | Record::Misc { title, .. }) = rec;
    fix_proper_nouns(title);
    if let Record::Inproceedings { booktitle, .. } | Record::Incollection { booktitle, .. } = rec {
        fix_proper_nouns(booktitle);
    }
}

/// Wraps acronyms such as `SAT` of `MaxSAT` in curly braces
fn fix_acronyms(string: &mut String) {
    let mut changed = None;
    let mut offset = 0;
    for matched in HYPHENATED_WORD_PATTERN.find_iter(string) {
        // Acronym cases:
        // 1. has more than one upper case and not all of them are after a dash
        // 2. starts with lower case, but contains upper case
        // 3. contains an upper case letter and a numeric digit
        let first_upper = matched
            .as_str()
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or(false);
        let (n_upper, n_upper_after_dash, n_nums) = matched.as_str().chars().tuple_windows().fold(
            (if first_upper { 1 } else { 0 }, 0, 0),
            |(total, after_dash, nums), (first, second)| {
                (
                    total + u32::from(second.is_uppercase()),
                    after_dash + u32::from(first == '-' && second.is_uppercase()),
                    nums + u32::from(second.is_ascii_digit()),
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
        if n_upper - n_upper_after_dash > 1
            || (n_upper > 0 && n_nums > 0)
            || (!first_upper && n_upper > 0)
        {
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

fn fix_capital_after_colon(string: &mut String) {
    let mut changed = None;
    let mut offset = 0;
    for matched in CAPITAL_AFTER_COLON.find_iter(string) {
        let start = matched.end() + offset - 1;
        let end = matched.end() + offset;
        let changed = changed.get_or_insert_with(|| string.clone());
        // wrap in braces
        changed.insert(end, '}');
        changed.insert(start, '{');
        offset += 2;
    }
    if let Some(changed) = changed {
        *string = changed;
    }
}

pub fn capital_after_colon(rec: &mut Record) {
    let (Record::Article { title, .. }
    | Record::Proceedings { title, .. }
    | Record::Inproceedings { title, .. }
    | Record::Book { title, .. }
    | Record::Incollection { title, .. }
    | Record::Misc { title, .. }) = rec;
    fix_capital_after_colon(title);
    if let Record::Inproceedings { booktitle, .. } | Record::Incollection { booktitle, .. } = rec {
        fix_capital_after_colon(booktitle);
    }
}

fn fix_title_period(string: &mut String) {
    let trimmed = string.trim_end();
    if let Some(stripped) = trimmed.strip_suffix('.') {
        *string = stripped.to_string();
    }
}

pub fn strip_title_period(rec: &mut Record) {
    let (Record::Article { title, .. }
    | Record::Proceedings { title, .. }
    | Record::Inproceedings { title, .. }
    | Record::Book { title, .. }
    | Record::Incollection { title, .. }
    | Record::Misc { title, .. }) = rec;
    fix_title_period(title);
    if let Record::Inproceedings { booktitle, .. } | Record::Incollection { booktitle, .. } = rec {
        fix_title_period(booktitle);
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
        let rep = DATE_RANGE_PATTERN.replace(booktitle, "${1}${3}{\\textendash}${2}${4}");
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
    *title = title.replace(" - ", "{\\textemdash}");
    if let Record::Article { journal: venue, .. }
    | Record::Inproceedings {
        booktitle: venue, ..
    }
    | Record::Incollection {
        booktitle: venue, ..
    } = rec
    {
        *venue = venue.replace(" - ", "{\\textemdash}");
    }
    if let Record::Proceedings {
        publisher: Some(publisher),
        ..
    }
    | Record::Book {
        publisher: Some(publisher),
        ..
    }
    | Record::Misc {
        publisher: Some(publisher),
        ..
    } = rec
    {
        *publisher = publisher.replace(" - ", "{\\textemdash}");
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
    if rec.key() == "conf/tacas/JabsBBJ25"
        && let Record::Inproceedings { title, .. } = rec
    {
        *title =
            String::from("Certifying Pareto Optimality in Multi-objective Maximum Satisfiability");
    }
    // Smallcaps acronyms not properly in DBLP metadata
    if rec.key() == "conf/sat/DaviesB13"
        && let Record::Inproceedings { title, .. } = rec
    {
        *title = String::from("Exploiting the Power of {MIP} Solvers in {MAXSAT}");
    }
    // Incorrect year for JELIA proceedings
    if rec.key().starts_with("conf/jelia/2025")
        && let Record::Proceedings { year, .. } = rec
    {
        *year = 2025;
    }
    // Custom entries used in thesis
    if rec.key() == "conf/cp/JabsBIJ23"
        && let Record::Inproceedings { usera, .. } = rec
    {
        *usera = Some(String::from(
            "Proceedings of International Conference on Principles and Practice of Constraint Programming, {CP} 2023",
        ));
    }
    if rec.key() == "conf/cpaior/JabsBJ24"
        && let Record::Inproceedings { usera, .. } = rec
    {
        *usera = Some(String::from(
            "Proceedings of Integration of Constraint Programming, Artificial Intelligence, and Operations Research, {CPAIOR} 2024",
        ));
    }
    if rec.key() == "conf/tacas/JabsBBJ25"
        && let Record::Inproceedings { usera, .. } = rec
    {
        *usera = Some(String::from(
            "Proceedings of Tools and Algorithms for the Construction and Analysis of Systems, {TACAS} 2025",
        ));
    }
    if rec.key() == "conf/jelia/JabsBJ25"
        && let Record::Inproceedings { usera, .. } = rec
    {
        *usera = Some(String::from(
            "Logics in Artificial Intelligence, {JELIA} 2025",
        ));
    }
    if rec.key() == "conf/ijcai/ArgelichLS09"
        && let Record::Inproceedings { title, .. } = rec
    {
        *title = title.replace("Problemse", "Problems");
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

        let mut text = String::from("The Sat4j library, release 2.2.");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "The {Sat4j} library, release 2.2.");

        let mut text = String::from("A number is not an acronym 2024");
        super::fix_acronyms(&mut text);
        assert_eq!(text, "A number is not an acronym 2024");
    }

    #[test]
    fn proper_nouns() {
        let mut text = String::from("Translating Pseudo-Boolean Constraints into SAT");
        super::fix_proper_nouns(&mut text);
        assert_eq!(text, "Translating Pseudo-{Boolean} Constraints into SAT");

        let mut text =
            String::from("A Support-Based Algorithm for the Bi-Objective Pareto Constraint");
        super::fix_proper_nouns(&mut text);
        assert_eq!(
            text,
            "A Support-Based Algorithm for the Bi-Objective {Pareto} Constraint"
        );
    }

    #[test]
    fn capital_after_colon() {
        let mut text =
            String::from("The Seesaw Algorithm: Function Optimization Using Implicit Hitting Sets");
        super::fix_capital_after_colon(&mut text);
        assert_eq!(
            text,
            "The Seesaw Algorithm: {F}unction Optimization Using Implicit Hitting Sets"
        );

        let mut text = String::from("Non: capitalized");
        super::fix_capital_after_colon(&mut text);
        assert_eq!(text, "Non: capitalized");
    }
}

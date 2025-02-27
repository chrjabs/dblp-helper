use lazy_static::lazy_static;
use regex::Regex;

use crate::dblp::Record;

mod unicode;

lazy_static! {
    static ref RANGE_PATTERN: Regex = Regex::new(r"(\d)-(\d)").unwrap();
    static ref AUTHOR_NUM_PATTERN: Regex = Regex::new(r" \d\d\d\d$").unwrap();
}

pub fn page_range(rec: &mut Record) {
    match rec {
        Record::Article { pages, .. }
        | Record::Inproceedings { pages, .. }
        | Record::Incollection { pages, .. } => {
            if let Some(pages) = pages {
                let rep = RANGE_PATTERN.replace(pages, "${1}--${2}");
                let _ = std::mem::replace(pages, rep.to_string());
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
                let _ = std::mem::replace(author, rep.to_string());
            }
        }
        _ => {}
    }
    match rec {
        Record::Proceedings { editor, .. }
        | Record::Inproceedings { editor, .. }
        | Record::Book { editor, .. }
        | Record::Incollection { editor, .. } => {
            for editor in editor.iter_mut() {
                let rep = AUTHOR_NUM_PATTERN.replace(editor, "");
                let _ = std::mem::replace(editor, rep.to_string());
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
            unicode::replace(series);
            unicode::replace(publisher);
        }
        Record::Inproceedings {
            author,
            title,
            editor,
            booktitle,
            series,
            publisher,
            ..
        }
        | Record::Incollection {
            author,
            title,
            editor,
            booktitle,
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
            unicode::replace(series);
            unicode::replace(publisher);
            unicode::replace(booktitle);
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
            unicode::replace(series);
            unicode::replace(publisher);
        }
    }
}

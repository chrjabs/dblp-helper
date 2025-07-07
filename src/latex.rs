use std::{fs, io};

pub struct CiteKeyIter {
    aux_stack: Vec<AuxFileIter<io::BufReader<fs::File>>>,
    base_path: camino::Utf8PathBuf,
    follow_inputs: bool,
}

impl CiteKeyIter {
    pub fn new<P: AsRef<camino::Utf8Path>>(aux_file: P, follow_inputs: bool) -> io::Result<Self> {
        let aux_file = aux_file.as_ref();
        let mut base_path = aux_file.to_path_buf();
        base_path.pop();
        let aux_iter =
            AuxFileIter::new(io::BufReader::new(fs::File::open(aux_file.as_std_path())?));
        Ok(CiteKeyIter {
            aux_stack: vec![aux_iter],
            base_path,
            follow_inputs,
        })
    }
}

impl Iterator for CiteKeyIter {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let last = self.aux_stack.last_mut()?;
            let Some(aux_info) = last.next() else {
                self.aux_stack.pop();
                continue;
            };
            let aux_info = match aux_info {
                Ok(i) => i,
                Err(e) => return Some(Err(e)),
            };
            match aux_info {
                AuxFileInfo::CiteKey(key) => return Some(Ok(key)),
                AuxFileInfo::Input(path) => {
                    if !self.follow_inputs {
                        continue;
                    }
                    let mut aux_file = self.base_path.clone();
                    aux_file.push(&path);
                    let aux_file = match fs::File::open(&aux_file) {
                        Ok(f) => f,
                        Err(error) => {
                            // soft fail of error when reading input file
                            crate::cli::warning!(
                                "LaTeX",
                                "failed to open sub aux file `{path}`: `{error}`"
                            );
                            continue;
                        }
                    };
                    let aux_iter = AuxFileIter::new(io::BufReader::new(aux_file));
                    self.aux_stack.push(aux_iter);
                }
            }
        }
    }
}

enum AuxFileInfo {
    CiteKey(String),
    Input(camino::Utf8PathBuf),
}

struct AuxFileIter<R> {
    reader: R,
    buffer: String,
}

impl<R: io::BufRead> AuxFileIter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
        }
    }

    fn yield_citekey(&mut self) -> String {
        // `self.buffer` has citation line with `\citation{` or `\abx@aux@cite{<>}{` already
        // stripped
        let sep = self.buffer.find(',').unwrap_or_else(|| {
            self.buffer
                .find('}')
                .unwrap_or_else(|| panic!("invalid line in `.aux` file"))
        });
        let rest = self.buffer.split_off(sep + 1);
        // remove `}` or `,`
        self.buffer.pop();
        std::mem::replace(&mut self.buffer, rest)
    }
}

impl<R: io::BufRead> Iterator for AuxFileIter<R> {
    type Item = io::Result<AuxFileInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.trim().is_empty() {
            self.buffer.clear();
            loop {
                match self.reader.read_line(&mut self.buffer) {
                    Ok(read) => {
                        if read == 0 {
                            return None;
                        }
                        // latex-native citation
                        if self.buffer.starts_with(r"\citation{") {
                            self.buffer.drain(..10);
                            return Some(Ok(AuxFileInfo::CiteKey(self.yield_citekey())));
                        }
                        // biblatex citation
                        if self.buffer.starts_with(r"\abx@aux@cite{") {
                            self.buffer.drain(..14);
                            let strip_till = self
                                .buffer
                                .find('{')
                                .unwrap_or_else(|| panic!("invalid line in `.aux` file"));
                            self.buffer.drain(..=strip_till);
                            return Some(Ok(AuxFileInfo::CiteKey(self.yield_citekey())));
                        }
                        // input line for another aux file
                        if self.buffer.starts_with(r"\@input{") {
                            self.buffer.drain(..8);
                            self.buffer.drain(self.buffer.trim_end().len()..);
                            debug_assert_eq!(self.buffer.chars().next_back(), Some('}'));
                            self.buffer.pop();
                            return Some(Ok(AuxFileInfo::Input(camino::Utf8PathBuf::from(
                                std::mem::take(&mut self.buffer),
                            ))));
                        }
                        // skip all other lines
                        self.buffer.clear();
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
        }
        Some(Ok(AuxFileInfo::CiteKey(self.yield_citekey())))
    }
}

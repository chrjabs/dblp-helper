use std::io;

pub struct CiteKeyIter<R> {
    reader: R,
    buffer: String,
}

impl<R: io::BufRead> CiteKeyIter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
        }
    }
}

impl<R: io::BufRead> Iterator for CiteKeyIter<R> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.trim().is_empty() {
            self.buffer.clear();
            loop {
                match self.reader.read_line(&mut self.buffer) {
                    Ok(read) => {
                        if read == 0 {
                            return None;
                        }
                        if self.buffer.starts_with("\\citation{") {
                            self.buffer.drain(..10);
                            break;
                        }
                        // skip non-citation line
                        self.buffer.clear();
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
        }

        // `self.buffer` has citation line with `\citation{` already stripped
        let sep = self.buffer.find(',').unwrap_or_else(|| {
            self.buffer
                .find('}')
                .unwrap_or_else(|| panic!("invalid line in `.aux` file"))
        });
        let rest = self.buffer.split_off(sep + 1);
        self.buffer.pop();
        Some(Ok(std::mem::replace(&mut self.buffer, rest)))
    }
}

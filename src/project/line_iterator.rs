use std::io;
use std::io::BufRead;

/// Iterator that preserves the newlines in the yielded values.
pub struct LineIterator<T: BufRead> {
    pub reader: T,
}

impl<T: BufRead> LineIterator<T> {
    pub fn new(reader: T) -> Self {
        Self { reader }
    }
}

impl<T: BufRead> Iterator for LineIterator<T> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::new();
        match self.reader.read_until(b'\n', &mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

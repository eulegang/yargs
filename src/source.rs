use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Stdin, StdinLock};

impl From<File> for Source {
    fn from(f: File) -> Source {
        Source::File(f)
    }
}

impl Default for Source {
    fn default() -> Source {
        Source::Stdin(io::stdin())
    }
}

pub enum Source {
    File(File),
    Stdin(Stdin),
}

impl Source {
    pub fn buffer(&mut self) -> SourceBuffer {
        match self {
            Source::File(f) => SourceBuffer::File(BufReader::new(f)),
            Source::Stdin(s) => SourceBuffer::Stdin(s.lock()),
        }
    }
}

pub enum SourceBuffer<'a> {
    File(BufReader<&'a mut File>),
    Stdin(StdinLock<'a>),
}

impl BufRead for SourceBuffer<'_> {
    fn consume(&mut self, amt: usize) {
        match self {
            SourceBuffer::File(f) => f.consume(amt),
            SourceBuffer::Stdin(s) => s.consume(amt),
        }
    }

    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            SourceBuffer::File(f) => f.fill_buf(),
            SourceBuffer::Stdin(s) => s.fill_buf(),
        }
    }
}

impl Read for SourceBuffer<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            SourceBuffer::File(f) => f.read(buf),
            SourceBuffer::Stdin(s) => s.read(buf),
        }
    }
}

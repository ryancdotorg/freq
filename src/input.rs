use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub struct Input<'a> {
    pub label: String,
    inner: Box<dyn BufRead + 'a>,
}

impl<'a> Input<'a> {
    pub fn stdin() -> io::Result<Input<'a>> {
        Input::reader(io::stdin().lock(), "STDIN")
    }

    pub fn path(path: &str) -> io::Result<Input<'a>> {
        File::open(path).map(|file| Input::file(file, path).unwrap())
    }

    pub fn file(file: File, path: &str) -> io::Result<Input<'a>> {
        Input::reader(BufReader::new(file), path)
    }

    pub fn reader(reader: impl BufRead + 'a, label: &str) -> io::Result<Input<'a>> {
        Ok(Input {
            label: label.to_string(),
            inner: Box::new(reader),
        })
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}

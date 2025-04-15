use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

#[cfg(feature = "unbz2")]
use bzip2::bufread::MultiBzDecoder;
#[cfg(feature = "unbz2")]
const BZIP2_MAGIC: [u8; 3] = *b"BZh";

#[cfg(feature = "ungz")]
use flate2::bufread::MultiGzDecoder;
#[cfg(feature = "ungz")]
const GZIP_MAGIC:  [u8; 3] = [0x1f, 0x8b, 0x08];

#[cfg(feature = "unxz")]
use xz2::read::XzDecoder;
#[cfg(feature = "unxz")]
const XZ_MAGIC:    [u8; 6] = *b"\xfd7zXZ\0";

#[cfg(feature = "unzstd")]
use zstd::stream::read::Decoder as ZstdDecoder;
#[cfg(feature = "unzstd")]
const ZSTD_MAGIC:  [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

pub struct Input<'a> {
    pub label: String,
    inner: Box<dyn BufRead + 'a>,
}

impl<'a> Input<'a> {
    // stdin -> reader
    pub fn stdin() -> io::Result<Input<'a>> {
        Input::reader(io::stdin().lock(), "STDIN")
    }

    // path -> file -> reader
    pub fn path<T: AsRef<Path>>(path: &T) -> io::Result<Input<'a>> {
        let path: &Path = path.as_ref();
        let label = path.as_os_str().to_string_lossy();
        File::open(path).map(|file| Input::file(file, &label).unwrap())
    }

    // file -> reader
    pub fn file<T: Display>(file: File, label: T) -> io::Result<Input<'a>> {
        Input::reader(BufReader::new(file), label)
    }

    #[cfg(feature = "_decompress")]
    fn with_buffer<R: Read + 'a, T: Display>(read: R, label: T) -> io::Result<Input<'a>> {
        Ok(Input { label: label.to_string(), inner: Box::new(BufReader::new(read)), })
    }

    #[cfg(feature = "_decompress")]
    pub fn reader<T: Display>(mut reader: impl BufRead + 'a, label: T) -> io::Result<Input<'a>> {
        let buf = reader.fill_buf()?;

        match 1 {
            #[cfg(feature = "unbz2")]
            _ if buf.starts_with(&BZIP2_MAGIC) => Input::with_buffer(MultiBzDecoder::new(reader), label),
            #[cfg(feature = "ungz")]
            _ if buf.starts_with(&GZIP_MAGIC) => Input::with_buffer(MultiGzDecoder::new(reader), label),
            #[cfg(feature = "unxz")]
            _ if buf.starts_with(&XZ_MAGIC) => Input::with_buffer(XzDecoder::new(reader), label),
            #[cfg(feature = "unzstd")]
            _ if buf.starts_with(&ZSTD_MAGIC) => Input::with_buffer(ZstdDecoder::with_buffer(reader)?, label),
            _ => Ok(Input { label: label.to_string(), inner: Box::new(reader) }),
        }
    }

    #[cfg(not(feature = "_decompress"))]
    pub fn reader<T: Display>(reader: impl BufRead + 'a, label: T) -> io::Result<Input<'a>> {
        Ok(Input { label: label.to_string(), inner: Box::new(reader) })
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}

impl Read for Input<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl BufRead for Input<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}

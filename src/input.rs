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

#[cfg(feature = "unlz4")]
use lz4_flex::frame::FrameDecoder;
#[cfg(feature = "unlz4")]
const LZ4_MAGIC: [u8; 4] = [0x04, 0x22, 0x4d, 0x18];

#[cfg(feature = "unxz")]
use xz2::read::XzDecoder;
#[cfg(feature = "unxz")]
const XZ_MAGIC:    [u8; 6] = *b"\xfd7zXZ\0";

#[cfg(feature = "unzstd")]
use zstd::stream::read::Decoder as ZstdDecoder;
#[cfg(feature = "unzstd")]
const ZSTD_MAGIC:  [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];

pub struct Input<'a> {
    inner: Box<dyn BufRead + 'a>,
    pub label: String,
    pub format: Option<&'static str>,
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

    #[cfg(feature = "_any_decompress")]
    fn with_buffer<R: Read + 'a, T: Display>(read: R, label: T, format: &'static str) -> io::Result<Input<'a>> {
        Ok(Input { inner: Box::new(BufReader::new(read)), label: label.to_string(), format: Some(format) })
    }

    #[cfg(feature = "_any_decompress")]
    pub fn reader<T: Display>(mut reader: impl BufRead + 'a, label: T) -> io::Result<Input<'a>> {
        let buf = reader.fill_buf()?;

        match 1 {
            #[cfg(feature = "unbz2")]
            _ if buf.starts_with(&BZIP2_MAGIC) => Input::with_buffer(MultiBzDecoder::new(reader), label, "bzip2"),
            #[cfg(feature = "ungz")]
            _ if buf.starts_with(&GZIP_MAGIC) => Input::with_buffer(MultiGzDecoder::new(reader), label, "gzip"),
            #[cfg(feature = "unlz4")]
            _ if buf.starts_with(&LZ4_MAGIC) => Input::with_buffer(FrameDecoder::new(reader), label, "lz4"),
            #[cfg(feature = "unxz")]
            _ if buf.starts_with(&XZ_MAGIC) => Input::with_buffer(XzDecoder::new(reader), label, "xz"),
            #[cfg(feature = "unzstd")]
            _ if buf.starts_with(&ZSTD_MAGIC) => Input::with_buffer(ZstdDecoder::with_buffer(reader)?, label, "zstd"),
            _ => Ok(Input { inner: Box::new(reader), label: label.to_string(), format: None }),
        }
    }

    #[cfg(not(feature = "_any_decompress"))]
    pub fn reader<T: Display>(reader: impl BufRead + 'a, label: T) -> io::Result<Input<'a>> {
        Ok(Input { inner: Box::new(reader), label: label.to_string(), format: None, })
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    #[allow(dead_code)]
    pub fn get_format(&self) -> &Option<&'static str> {
        &self.format
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

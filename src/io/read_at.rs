use crate::io;

use std::convert::*;
use std::fs::File;
use std::io::Read;



pub trait ReadAt {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;

    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        let len64 = u64::try_from(buf.len()).map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "buf.len() exceeds u64 range"))?;
        let _end = offset.checked_add(len64).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "offset + buf.len() exceeds u64 range"))?;

        let mut buf = buf;
        let mut offset = offset;
        while !buf.is_empty() {
            let read = self.read_at(buf, offset)?;
            if read == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached before buf filled"));
            }
            buf = &mut buf[read..];
            offset += read as u64;
        }
        Ok(())
    }

    fn read_exact_at_advance(&self, buf: &mut [u8], offset: &mut u64) -> io::Result<()> {
        self.read_exact_at(buf, *offset)?;
        *offset = offset.checked_add(buf.len() as u64).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "read past u64::MAX"))?;
        Ok(())
    }

    fn read_until_at(&self, byte: u8, buf: &mut Vec<u8>, offset: u64) -> io::Result<usize> {
        let start = buf.len();

        let mut offset = offset;
        loop {
            let start = buf.len();
            buf.resize(start + 1024, 0u8);
            match self.read_at(&mut buf[start..], offset) {
                Err(err) => {
                    buf.resize(start, 0u8);
                    return Err(err);
                },
                Ok(0) => break,
                Ok(n) => {
                    buf.resize(start + n, 0u8);
                    if let Some(nul) = buf[start .. start+n].iter().position(|b| *b == byte) {
                        buf.resize(start + nul + 1, 0u8);
                        break;
                    }
                    let n = u64::try_from(n).map_err(|_| io::Error::new(io::ErrorKind::UnexpectedEof, "read exceeded u64 range"))?;
                    offset = offset.checked_add(n).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "offset exceeded u64 range"))?;
                },
            }
        }

        Ok(buf.len() - start)
    }

    fn read_asciiz_at<'b>(&'_ self, buf: &'b mut Vec<u8>, offset: u64) -> io::Result<&'b [u8]> {
        let _ = self.read_until_at(0, buf, offset)?;
        Ok(buf[..].strip_suffix(&[0]).unwrap_or(&buf[..]))
    }
}

impl<R: ReadAt> ReadAt for &R {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        (**self).read_at(buf, offset)
    }
}



pub struct ReadAtFile {
    #[cfg(not(any(unix, windows, target_env="wasi")))] file: RefCell<File>,
    #[cfg(    any(unix, windows, target_env="wasi") )] file: File,
}

impl ReadAtFile {
    pub fn new(file: File) -> Self { Self::new_impl(file) }
}

impl From<File> for ReadAtFile {
    fn from(file: File) -> Self { Self::new(file) }
}

impl ReadAt for ReadAtFile {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> { self.read_at_impl(buf, offset) }
}



#[cfg(unix)] impl ReadAtFile {
    fn new_impl(file: File) -> Self { Self { file } }
    fn read_at_impl(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> { std::os::unix::fs::FileExt::read_at(&self.file, buf, offset) }
}

#[cfg(windows)] impl ReadAtFile {
    fn new_impl(file: File) -> Self { Self { file } }
    fn read_at_impl(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> { std::os::windows::fs::FileExt::seek_read(&self.file, buf, offset) }
}

#[cfg(target_env="wasi")] impl ReadAtFile {
    fn new_impl(file: File) -> Self { Self { file } }
    fn read_at_impl(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> { std::os::wasi::fs::FileExt::read_at(&self.file, buf, offset) }
}

#[cfg(not(any(unix, windows, target_env="wasi")))] impl ReadAtFile {
    fn new_impl(file: File) -> Self { Self { file: RefCell::new(file) } }
    fn read_at_impl(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        use io::*;
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(offset))?;
        file.read(buf)
    }
}



pub struct ReadAtReader<R: ReadAt> {
    read_at:    R,
    pos:        u64,
}

impl<R: ReadAt> ReadAtReader<R> {
    pub fn new(read_at: R, pos: u64) -> Self { Self { read_at, pos } }
}

impl<R: ReadAt> Read for ReadAtReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.read_at.read_at(buf, self.pos)?;
        self.pos += read as u64;
        Ok(read)
    }
}

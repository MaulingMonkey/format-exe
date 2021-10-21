use crate::io;

use maulingmonkey_io_adapters::ReadAt;

use std::convert::*;



pub trait ReadAtExt : ReadAt {
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

impl<R: ReadAt> ReadAtExt for R {}

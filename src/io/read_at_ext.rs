use crate::io;

use maulingmonkey_io_adapters::ReadAt;

use std::convert::*;



/// Additional convenience methods for [`ReadAt`]
pub trait ReadAtExt : ReadAt {
    /// Read `buf.len()` bytes, overwriting `buf`, from offset `*offset`, advancing `*offset` by the same amount.
    ///
    /// *   `buf[..]` *may* have been modified on [`io::Error`]
    /// *   `*offset` will *not* advance on [`io::Error`]
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::UnexpectedEof`] if `*offset` would overflow
    /// *   [`io::Error`] if the underlying [`ReadAt::read_exact_at`]s failed
    fn read_exact_at_advance(&self, buf: &mut [u8], offset: &mut u64) -> io::Result<()> {
        let final_offset = offset.checked_add(buf.len() as u64).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "would read past u64::MAX"))?;
        self.read_exact_at(buf, *offset)?;
        *offset = final_offset;
        Ok(())
    }

    /// Append read bytes into `buf`, from `offset`, until `byte` is encountered.
    ///
    /// *   The original contents of `buf` are not cleared
    /// *   `buf` *may* have been appended to on [`io::Error`]
    /// *   `buf` should end with `byte` on [`Ok`]
    /// *   Over-reads of `self` may occur (extra data will be discarded)
    /// *   The seek position of `self` is left indeterminite
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::UnexpectedEof`] if `offset` would overflow
    /// *   [`io::Error`] if the underlying [`ReadAt::read_at`]s failed
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

    /// Append read `\0`-terminated string into `buf`, from `offset`.
    ///
    /// *   The original contents of `buf` are not cleared
    /// *   `buf` *may* have been appended to on [`io::Error`]
    /// *   `buf` will *not* contain the terminal `\0` on [`Ok`] (unless `buf` already ended with `\0` and this read/appended an empty string)
    /// *   Over-reads of `self` may occur (extra data will be discarded)
    /// *   The seek position of `self` is left indeterminite
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::UnexpectedEof`] if `offset` would overflow
    /// *   [`io::Error`] if the underlying [`ReadAt::read_at`]s failed
    fn read_asciiz_at<'b>(&'_ self, buf: &'b mut Vec<u8>, offset: u64) -> io::Result<&'b [u8]> {
        let _ = self.read_until_at(0, buf, offset)?;
        Ok(buf[..].strip_suffix(&[0]).unwrap_or(&buf[..]))
    }
}

impl<R: ReadAt> ReadAtExt for R {}

use crate::io::{self, *};



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

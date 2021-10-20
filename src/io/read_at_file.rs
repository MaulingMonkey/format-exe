use crate::io;

use std::convert::*;
use std::fs::File;



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

impl io::ReadAt for ReadAtFile {
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

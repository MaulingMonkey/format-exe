use crate::io;



/// Encountered the end of the file
pub struct Eof;

impl From<Eof> for io::Error {
    fn from(_: Eof) -> io::Error {
        io::Error::from(io::ErrorKind::UnexpectedEof)
    }
}

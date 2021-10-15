use bytemuck::*;

use std::convert::*;
use std::io;



pub trait FromMemory : Sized {
    type Raw : Default + Zeroable + Pod;
    type Error : Into<io::Error> + From<EofError>;

    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error>;

    fn from_memory(mem: &mut &[u8]) -> Result<Self, Self::Error> {
        let mut raw = Self::Raw::default();
        let dst = bytes_of_mut(&mut raw);
        if let Some(src) = mem.get(..dst.len()) {
            dst.copy_from_slice(src);
            *mem = &mem[dst.len()..];
            Self::from_raw(raw)
        } else {
            Err(Self::Error::from(EofError))
        }
    }

    fn from_io(read: &mut impl io::Read) -> io::Result<Self> {
        let mut raw = Self::Raw::default();
        read.read_exact(bytes_of_mut(&mut raw))?;
        Self::from_raw(raw).map_err(|e| e.into())
    }
}



pub struct EofError;

impl From<EofError> for io::Error {
    fn from(_: EofError) -> io::Error {
        io::Error::from(io::ErrorKind::UnexpectedEof)
    }
}



macro_rules! from_memory_integer {
    ( $($integer:ident),* ) => {
        $(
            impl FromMemory for $integer {
                type Raw    = [u8; std::mem::size_of::<$integer>()];
                type Error  = std::io::Error;

                fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> {
                    Ok(Self::from_le_bytes(raw))
                }
            }
        )*
    };
}

from_memory_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

use crate::*;
use crate::io::ReadAt;

use abistr::CStrBuf;
use bytemuck::*;

use std::convert::*;
use std::num::*;
use std::io;



#[doc(hidden)]
pub trait FromMemory : Sized {
    type Raw    : Default + Zeroable + Pod;
    type Error  : Into<io::Error> + From<error::Eof>;

    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error>;

    fn from_memory(mem: &mut &[u8]) -> Result<Self, Self::Error> {
        let mut raw = Self::Raw::default();
        let dst = bytes_of_mut(&mut raw);
        if let Some(src) = mem.get(..dst.len()) {
            dst.copy_from_slice(src);
            *mem = &mem[dst.len()..];
            Self::from_raw(raw)
        } else {
            Err(Self::Error::from(error::Eof))
        }
    }

    fn from_io(read: &mut impl io::Read) -> io::Result<Self> {
        let mut raw = Self::Raw::default();
        read.read_exact(bytes_of_mut(&mut raw))?;
        Self::from_raw(raw).map_err(|err| err.into())
    }

    fn from_read_at(read_at: &impl ReadAt, offset: u64) -> io::Result<Self> {
        let mut raw = Self::Raw::default();
        read_at.read_exact_at(bytes_of_mut(&mut raw), offset)?;
        Self::from_raw(raw).map_err(|err| err.into())
    }

    fn from_read_at_advance(read_at: &impl ReadAt, offset: &mut u64) -> io::Result<Self> {
        let mut raw = Self::Raw::default();
        read_at.read_exact_at(bytes_of_mut(&mut raw), *offset)?;
        let s = Self::from_raw(raw).map_err(|err| err.into())?;
        *offset += std::mem::size_of::<Self::Raw>() as u64;
        Ok(s)
    }
}



impl FromMemory for () {
    type Raw    = [u8; 0];
    type Error  = std::io::Error;
    fn from_raw(_: Self::Raw) -> Result<(), Self::Error> { Ok(()) }
}

impl<B: Default + Pod> FromMemory for CStrBuf<B> {
    type Raw    = Self;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(raw) }
}

macro_rules! from_memory_le_integers {
    ( $($le:ty => $int:ty),* $(,)? ) => {
        $(
            impl FromMemory for $int {
                type Raw    = $le;
                type Error  = std::io::Error;
                fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(raw.to_le()) }
            }
        )*
    };
}

from_memory_le_integers! {
    i8le    => i8,
    i16le   => i16,
    i32le   => i32,
    i64le   => i64,
    i128le  => i128,
    u8le    => u8,
    u16le   => u16,
    u32le   => u32,
    u64le   => u64,
    u128le  => u128,
}

macro_rules! from_memory_le_opt_nz_integers {
    ( $($le:ty => Option<$nz:ty>),* $(,)? ) => {
        $(
            impl FromMemory for Option<$nz> {
                type Raw    = $le;
                type Error  = std::io::Error;
                fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(<$nz>::new(raw.to_le())) }
            }
        )*
    };
}

from_memory_le_opt_nz_integers! {
    i8le    => Option<NonZeroI8>,
    i16le   => Option<NonZeroI16>,
    i32le   => Option<NonZeroI32>,
    i64le   => Option<NonZeroI64>,
    i128le  => Option<NonZeroI128>,
    u8le    => Option<NonZeroU8>,
    u16le   => Option<NonZeroU16>,
    u32le   => Option<NonZeroU32>,
    u64le   => Option<NonZeroU64>,
    u128le  => Option<NonZeroU128>,
}

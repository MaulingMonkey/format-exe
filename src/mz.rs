//! The MS-DOS EXE format, also known as MZ after its signature (the initials of Microsoft engineer **M**ark **Z**bykowski)
//!
//! ## References
//! *   <https://wiki.osdev.org/MZ>
//! *   <https://en.wikipedia.org/wiki/DOS_MZ_executable>

use crate::*;

use bytemuck::*;

use std::fmt::{self, *};
use std::io::{self, Read};



#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub struct Header {
    pub signature:          abistr::CStrBuf<[u8; 2]>,
    pub last_page:          Bytes,
    pub pages:              Pages,
    pub nrelocs:            u16,
    pub hdrsize:            Paragraphs,
    pub minalloc:           Paragraphs,
    pub maxalloc:           Paragraphs,
    pub ss:                 u16,
    pub sp:                 u16,
    pub checksum:           u16,
    pub ip:                 u16,
    pub cs:                 u16,
    pub relocs:             u16,
    pub overlay:            u16,

    // PE extensions
    pub _reserved_a:        Reserved<8>,
    pub oem_id:             u16,
    pub oem_info:           u16,
    pub _reserved_b:        Reserved<20>,
    pub pe_header_start:    u32,

    pub _non_exhaustive:    (),
}

/// { pub [offset](struct.Relocation.html#structfield.offset): [u16], pub [segment](struct.Relocation.html#structfield.segment): [u16] }
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub struct Relocation {
    pub offset:             u16,
    pub segment:            u16,
}

/// 1 **[Pages]** = 32 [Paragraphs] = 512 [Bytes]
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Pages(u16);

/// 1 [Pages] = 32 **[Paragraphs]** = 512 [Bytes]
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Paragraphs(u16);

/// 1 [Pages] = 32 [Paragraphs] = 512 **[Bytes]**
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Bytes(u16);



impl Header {
    /// Reads an [`mz::Header`] from `read`
    ///
    /// ## Errors
    /// * [`io::ErrorKind::InvalidData`] if [`signature`](#structfield.signature) ≠ `"MZ"`
    /// * [`io::ErrorKind::UnexpectedEof`] if `read` didn't contain enough data
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        Ok(raw::Header::read_from(read)?.into())
    }
}

impl Relocation {
    /// Reads an [`mz::Relocation`] from `read`
    ///
    /// ## Errors
    /// * [`io::ErrorKind::UnexpectedEof`] if `read` didn't contain enough data
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        Ok(raw::Relocation::read_from(read)?.into())
    }
}

impl Pages {
    pub fn pages<N: From<u16>>(&self)         -> N { N::from(self.0) }
    pub fn paragraphs<N: From<u32>>(&self)    -> N { N::from(self.0 as u32 * (512 / 16)) }
    pub fn bytes<N: From<u32>>(&self)         -> N { N::from(self.0 as u32 * 512) }
}

impl Paragraphs {
    pub fn paragraphs<N: From<u16>>(&self)    -> N { N::from(self.0) }
    pub fn bytes<N: From<u32>>(&self)         -> N { N::from(self.0 as u32 * 16) }
}

impl Bytes {
    pub fn bytes<N: From<u16>>(&self)         -> N { N::from(self.0) }
}



impl From<raw::Header> for Header {
    fn from(raw: raw::Header) -> Self {
        Self {
            signature:          raw.signature.into(),
            last_page:          Bytes(raw.last_page.into()),
            pages:              Pages(raw.pages.into()),
            nrelocs:            raw.nrelocs.into(),
            hdrsize:            Paragraphs(raw.hdrsize.into()),
            minalloc:           Paragraphs(raw.minalloc.into()),
            maxalloc:           Paragraphs(raw.maxalloc.into()),
            ss:                 raw.ss.into(),
            sp:                 raw.sp.into(),
            checksum:           raw.checksum.into(),
            ip:                 raw.ip.into(),
            cs:                 raw.cs.into(),
            relocs:             raw.relocs.into(),
            overlay:            raw.overlay.into(),

            _reserved_a:        raw._reserved_a.into(),
            oem_id:             raw.oem_id.into(),
            oem_info:           raw.oem_info.into(),
            _reserved_b:        raw._reserved_b.into(),
            pe_header_start:    raw.pe_header_start.into(),

            _non_exhaustive:    raw._non_exhaustive.into(),
        }
    }
}

impl From<raw::Relocation> for Relocation {
    fn from(raw: raw::Relocation) -> Self {
        Self {
            offset:             raw.offset.into(),
            segment:            raw.segment.into(),
        }
    }
}

// TODO: From<Header> for raw::Header ?
// TODO: From<Relocation> for raw::Relocation ?

impl Debug for Bytes {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} bytes", self.0)
    }
}

impl Debug for Paragraphs {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} paragraphs", self.0)
    }
}

impl Debug for Pages {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{} pages", self.0)
    }
}



mod raw {
    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct Header {
        pub signature:          abistr::CStrBuf<[u8; 2]>,
        pub last_page:          u16le,
        pub pages:              u16le,
        pub nrelocs:            u16le,
        pub hdrsize:            u16le,
        pub minalloc:           u16le,
        pub maxalloc:           u16le,
        pub ss:                 u16le,
        pub sp:                 u16le,
        pub checksum:           u16le,
        pub ip:                 u16le,
        pub cs:                 u16le,
        pub relocs:             u16le,
        pub overlay:            u16le,

        // PE extensions
        pub _reserved_a:        Reserved<8>,
        pub oem_id:             u16le,
        pub oem_info:           u16le,
        pub _reserved_b:        Reserved<20>,
        pub pe_header_start:    u32le,

        pub _non_exhaustive:    (),
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct Relocation {
        pub offset:             u16le,
        pub segment:            u16le,
    }

    #[test] fn layout() {
        use std::mem::*;

        assert_eq!(size_of::<Header>(), 64);
        assert_eq!(size_of::<Relocation>(), 4);
        assert_eq!(align_of::<Header>(), align_of::<u32>());
        assert_eq!(align_of::<Relocation>(), align_of::<u16>());
    }

    // A paragraph is 16 bytes in size. A page (or block) is 512 bytes long.

    impl Header {
        pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
            let mut s = Self::default();
            read.read_exact(bytemuck::bytes_of_mut(&mut s))?;
            if s.signature.to_bytes() != b"MZ" {
                Err(io::Error::new(io::ErrorKind::InvalidData, "mz::Header::signature != \"MZ\""))
            } else {
                Ok(s)
            }
        }
    }

    impl Relocation {
        pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
            let mut s = Self::default();
            read.read_exact(bytemuck::bytes_of_mut(&mut s))?;
            Ok(s)
        }
    }
}

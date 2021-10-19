//! The MS-DOS EXE format, also known as MZ after its signature (the initials of Microsoft engineer **M**ark **Z**bykowski)
//!
//! ## References
//! *   <https://wiki.osdev.org/MZ>
//! *   <https://en.wikipedia.org/wiki/DOS_MZ_executable>

use crate::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read};



from_memory_struct! {
    /// The basic MZ/DOS header, found at offset 0 of `.exe` and `.dll` files.<br>
    /// For modern executables, the main field of note is [`pe_header_start`](struct.Header.html#structfield.pe_header_start): the PE header offset.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
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
        #[doc(hidden)] pub _reserved_a:        Reserved<8>,
        pub oem_id:             u16,
        pub oem_info:           u16,
        #[doc(hidden)] pub _reserved_b:        Reserved<20>,
        pub pe_header_start:    u32,

        #[doc(hidden)] pub _non_exhaustive:    (),
    }

    /// { pub [offset](struct.Relocation.html#structfield.offset): [u16], pub [segment](struct.Relocation.html#structfield.segment): [u16] }
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Relocation {
        pub offset:             u16,
        pub segment:            u16,
    }
}

// A paragraph is 16 bytes in size. A page (or block) is 512 bytes long.

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
        let s = Header::from_io(read)?;
        if s.signature.buffer() != b"MZ" {
            Err(io::Error::new(io::ErrorKind::InvalidData, "mz::Header::signature != \"MZ\""))
        } else {
            Ok(s)
        }
    }
}

impl Relocation {
    /// Reads an [`mz::Relocation`] from `read`
    ///
    /// ## Errors
    /// * [`io::ErrorKind::UnexpectedEof`] if `read` didn't contain enough data
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        Relocation::from_io(read)
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

impl FromMemory for Pages {
    type Raw    = u16le;
    type Error  = io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Pages(raw.to_le())) }
}

impl FromMemory for Paragraphs {
    type Raw    = u16le;
    type Error  = io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Paragraphs(raw.to_le())) }
}

impl FromMemory for Bytes {
    type Raw    = u16le;
    type Error  = io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Bytes(raw.to_le())) }
}



// TODO: From<Header> for raw::Header ?

impl Debug for Bytes        { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} bytes",       self.0) } }
impl Debug for Paragraphs   { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} paragraphs",  self.0) } }
impl Debug for Pages        { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{} pages",       self.0) } }



#[test] fn layout() {
    use std::mem::*;

    assert_eq!(size_of::<<Header as FromMemory>::Raw>(), 64);
    assert_eq!(size_of::<<Relocation as FromMemory>::Raw>(), 4);
    assert_eq!(align_of::<<Header as FromMemory>::Raw>(), align_of::<u32>());
    assert_eq!(align_of::<<Relocation as FromMemory>::Raw>(), align_of::<u16>());
}

//! The MS-DOS EXE/DLL format, also known as MZ after its signature (the initials of Microsoft engineer **M**ark **Z**bykowski)
//!
//! | Type  | Desc  |
//! | ------| ------|
//! | [`Header`]        | The basic MZ/DOS header, found at offset 0 of `.exe` and `.dll` files.<br>For modern binaries, the main field of note is [`pe_header_start`](struct.Header.html#structfield.pe_header_start) - the [`pe::Header`] offset.
//! | [`Relocation`]    | A relocation entry for 16-bit MS-DOS binaries.  Ignorable for modern 32/64-bit binaries.
//! | [`Bytes`]         | MZ binaries have many [`u16`] values in different units, these newtypes help avoid confusion.
//! | [`Pages`]         | <code>[Pages]\(1\) = [Bytes]\(16\)</code>
//! | [`Paragraphs`]    | <code>[Paragraphs]\(1\) = [Pages]\(32\) = [Bytes]\(512\)</code>
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
    /// For modern binaries, the main field of note is [`pe_header_start`](struct.Header.html#structfield.pe_header_start) - the [`pe::Header`] offset.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Header {
        /// `b"MZ"` for valid MZ/DOS modules.
        pub signature:          abistr::CStrBuf<[u8; 2]>,
        /// Size of the last page.
        pub last_page:          Bytes,
        /// Number of 512-byte pages.
        pub pages:              Pages,
        /// Number of 16-bit [`mz::Relocation`]s.
        pub nrelocs:            u16,
        /// Size of this [`mz::Header`] (+ any custom header data, [`mz::Relocation`]s?).  Does *not* include [`pe::Header`].
        pub hdrsize:            Paragraphs,
        /// Memory *required* by the program, excluding the [PSP](https://en.wikipedia.org/wiki/Program_Segment_Prefix) and image.
        pub minalloc:           Paragraphs,
        /// Memory *requested* by the program.
        pub maxalloc:           Paragraphs,
        /// Initial value of the 16-bit [Stack Segment](https://en.wikipedia.org/wiki/X86_memory_segmentation) register.
        pub ss:                 u16,
        /// Initial value of the 16-bit [Stack Pointer](https://en.wikipedia.org/wiki/X86_memory_segmentation) register.
        pub sp:                 u16,
        /// When added to the sum of all other [`u16`]s in the file, the result should be zero.
        pub checksum:           u16,
        /// Initial value of the 16-bit [Instruction Pointer](https://en.wikipedia.org/wiki/X86_memory_segmentation) register.
        pub ip:                 u16,
        /// Initial value of the 16-bit [Code Segment](https://en.wikipedia.org/wiki/X86_memory_segmentation) register.
        pub cs:                 u16,
        /// Offset from the start of this [`mz::Header`] (typically @ offset 0) to the first [`mz::Relocation`].
        pub relocs:             u16,
        /// Typically 0 for "the main executable.  See <https://retrocomputing.stackexchange.com/a/25742>.
        pub overlay:            u16,

        /// The fields below are [`pe`] extensions to the [`mz`] format, and may not be present in a more basic MS-DOS or Windows 3.1 executable.
        pub _pe_extensions:     (),

        // Reserved
        #[doc(hidden)] pub _reserved_a: Reserved<8>,
        /// Typically 0.
        pub oem_id:             u16,
        /// Typically 0.
        pub oem_info:           u16,
        // Reserved
        #[doc(hidden)] pub _reserved_b: Reserved<20>,
        /// Offset from the start of this [`mz::Header`] (typically @ offset 0) to the [`pe::Header`].
        pub pe_header_start:    u32,

        #[doc(hidden)] pub _non_exhaustive:    (),
    }

    /// A relocation entry for 16-bit MS-DOS binaries.
    /// Ignorable for modern 32/64-bit binaries.<br>
    /// <code>struct { pub [offset](struct.Relocation.html#structfield.offset): [u16], pub [segment](struct.Relocation.html#structfield.segment): [u16] }</code>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Relocation {
        pub offset:             u16,
        pub segment:            u16,
    }
}

// A paragraph is 16 bytes in size. A page (or block) is 512 bytes long.

/// ≈[`u16`] - 1 **[Pages]** = 32 [Paragraphs] = 512 [Bytes]
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Pages(u16);

/// ≈[`u16`] - 1 [Pages] = 32 **[Paragraphs]** = 512 [Bytes]
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Paragraphs(u16);

/// ≈[`u16`] - 1 [Pages] = 32 [Paragraphs] = 512 **[Bytes]**
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct Bytes(u16);



impl Header {
    /// Reads an [`mz::Header`] from `read`
    ///
    /// ## Errors
    /// * [`io::ErrorKind::InvalidData`] if [`signature`](#structfield.signature) ≠ `"MZ"`
    /// * No error if <code>sum([`checksum`](#structfield.checksum), ...entire file...)</code> ≠ `0`
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

use crate::*;
use crate::io::{self, *};
use super::*;

use maulingmonkey_io_adapters::{ReadAt, ReadAtCursor, ReadAtRef, SeeklessFile};

use std::convert::*;
use std::fmt::{self, *};
use std::fs::File;
use std::ops::*;
use std::path::PathBuf;



/// Wraps a [`ReadAt`] for ease of reading by caching [`pe::Header`], [`pe::SectionHeader`]s, etc.<br>
/// <br>
pub struct Reader<R> {
    src:                        Src,
    reader:                     R,
    mz_header:                  mz::Header,
    pe_header:                  pe::Header,
    pe_section_headers:         Vec<pe::SectionHeader>,
}

impl Reader<SeeklessFile> {
    /// Open a PE file and parse some basic headers.
    ///
    /// ### Example
    /// ```
    /// # use maulingmonkey_format_exe::*;
    /// let path = std::env::current_exe().expect("unable to get current_exe");
    /// let pe = pe::Reader::open(path).expect("error reading or parsing current_exe");
    /// let section0 = pe.pe_section_header(0).expect("current_exe has no sections");
    /// # if false { // no guarantee the first section is .text
    /// assert_eq!(".text", section0.name.to_string_lossy()); // not guaranteed but whatever
    /// # }
    /// ```
    pub fn open(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        let file = File::open(&path);
        let src = Src::PathBuf(path);
        Reader::read_src(SeeklessFile::from(src.anno(file, "error opening pe::Reader")?), src)
    }
}

impl<R: ReadAt> Reader<R> {
    /// Read a PE file and parse some basic headers.  Prefer [`Reader::open`] for better error messages.
    ///
    /// ### Example
    /// ```
    /// # use maulingmonkey_format_exe::*;
    /// let path = std::env::current_exe().expect("unable to get current_exe");
    /// let file = std::fs::File::open(path).expect("unable to open current_exe");
    /// let file = maulingmonkey_io_adapters::SeeklessFile::from(file);
    /// let pe = pe::Reader::read(file).expect("error reading or parsing current_exe");
    /// let section0 = pe.pe_section_header(0).expect("current_exe has no sections");
    /// # if false { // no guarantee the first section is .text
    /// assert_eq!(".text", section0.name.to_string_lossy()); // not guaranteed but whatever
    /// # }
    /// ```
    pub fn read(reader: R) -> io::Result<Self> { Self::read_src(reader, Src::Unknown) }

    /// Read the raw data referenced by a [`pe::SectionHeader`]
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::InvalidInput`] if `idx` >= `self.pe_section_headers().len()`
    /// *   [`io::ErrorKind::UnexpectedEof`] if the [`pe::SectionHeader`] references outside the underlying reader.
    /// *   [`io::Error`] forwarded from [`ReadAt::read_exact_at`]
    pub fn read_pe_section_data_by_idx(&self, idx: impl TryInto<usize>) -> io::Result<Vec<u8>> {
        self.read_pe_section_data(self.pe_section_header(idx).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "pe section idx out of bounds"))?)
    }

    /// Read the raw data referenced by a [`pe::SectionHeader`]
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::UnexpectedEof`] if the [`pe::SectionHeader`] references outside the underlying reader.
    /// *   [`io::Error`] forwarded from [`ReadAt::read_exact_at`]
    pub fn read_pe_section_data(&self, header: &pe::SectionHeader) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; usize::try_from(header.size_of_raw_data).expect("unable to allocate pe::SectionHeader::size_of_raw_data bytes")];
        self.read_pe_section_data_inplace(header, 0, &mut data)?;
        Ok(data)
    }

    /// Read the raw data referenced by a [`pe::SectionHeader`]
    ///
    /// May return less data than `data.len()`
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::UnexpectedEof`] if the [`pe::SectionHeader`] references outside the underlying reader.
    /// *   [`io::Error`] forwarded from [`ReadAt::read_exact_at`]
    pub fn read_pe_section_data_inplace<'a>(&'_ self, header: &'_ pe::SectionHeader, offset: u32, data: &'a mut [u8]) -> io::Result<&'a [u8]> {
        let ptr = u64::from(header.pointer_to_raw_data.map_or(0, |ptr| ptr.into())) + u64::from(offset);
        let n = usize::try_from(header.size_of_raw_data.saturating_sub(offset)).unwrap_or(!0).min(data.len());
        self.src.anno(self.reader.read_exact_at(&mut data[..n], ptr), "error reading PE section")?;
        Ok(&data[..n])
    }

    /// Read data from an [`RVA`] range from one or more [`pe::SectionHeader`].
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::InvalidInput`] if `rva` isn't entirely mapped by sections
    /// *   ...?
    pub fn read_exact_rva<'a>(&'_ self, rva: Range<RVA>, scratch: &'a mut Vec<u8>) -> io::Result<&'a [u8]> {
        scratch.resize(rva.end.to_usize() - rva.start.to_usize(), 0u8);

        let mut rva = rva.start;
        let mut o = &mut scratch[..];
        while !o.is_empty() {
            if let Some(section) = self.pe_section_headers.iter().find(|s| s.virtual_address_range().contains(&rva)).copied() {
                let n = self.read_pe_section_data_inplace(&section, rva - section.virtual_address, o)?.len();
                rva += n as u32;
                o = &mut o[n..];
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "not all RVA mapped")); // XXX
            }
        }

        Ok(&scratch[..])
    }

    /// Read a `b'\0'`-terminated string starting from `rva` using `scratch` as a temp buffer.
    ///
    /// ### Errors
    /// *   ...?
    pub fn read_strz_rva<'a>(&'_ self, rva: RVA, scratch: &'a mut Vec<u8>) -> io::Result<&'a [u8]> {
        scratch.clear();
        BufReader::new(RvaReader::new(self, rva)).read_until(0, scratch)?;
        Ok(scratch[..].strip_suffix(&[0]).unwrap_or(&scratch[..]))
    }

    /// Read a `b'\0'`-terminated UTF8 string starting from `rva` using `scratch` as a temp buffer.
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::InvalidData`] if the string wasn't UTF8
    pub fn read_utf8z_rva<'a>(&'_ self, rva: RVA, scratch: &'a mut Vec<u8>) -> io::Result<&'a str> {
        Ok(std::str::from_utf8(self.read_strz_rva(rva, scratch)?).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?)
    }

    /// Read a `b'\0'`-terminated ASCII (7-bit) string starting from `rva` using `scratch` as a temp buffer.
    ///
    /// ### Errors
    /// *   [`io::ErrorKind::InvalidData`] if the string wasn't ASCII (7-bit)
    pub fn read_asciiz_rva<'a>(&'_ self, rva: RVA, scratch: &'a mut Vec<u8>) -> io::Result<&'a str> {
        let s = self.read_utf8z_rva(rva, scratch)?;
        if !s.is_ascii() { return Err(io::Error::new(io::ErrorKind::InvalidData, "string is not ASCII")); }
        Ok(s)
    }

    fn read_src(reader: R, src: Src) -> io::Result<Self> {
        let mz_header = src.anno(mz::Header::from_read_at(&reader, 0), "error reading mz::Header")?;

        let mut pe_reader = ReadAtCursor::new(ReadAtRef(&reader), u64::from(mz_header.pe_header_start), !0);
        let pe_header = src.anno(pe::Header::read_from(&mut pe_reader), "error reading pe::Header")?;

        let mut pe_section_headers = vec![pe::SectionHeader::default(); pe_header.file_header.nsections.into()];
        for section in &mut pe_section_headers {
            *section = src.anno(pe::SectionHeader::read_from(&mut pe_reader), "error reading pe::SectionHeader")?;
        }

        Ok(Self {
            src,
            reader,
            mz_header,
            pe_header,
            pe_section_headers,
        })
    }
}

impl<R> Reader<R> {
    // #[allow(missing_docs)]: Honestly these are all pretty self-explanatory
    #[allow(missing_docs)] pub fn mz_header(&self) -> &mz::Header { &self.mz_header }
    #[allow(missing_docs)] pub fn pe_header(&self) -> &pe::Header { &self.pe_header }
    #[allow(missing_docs)] pub fn pe_section_headers(&self) -> &[pe::SectionHeader] { &self.pe_section_headers[..] }

    #[allow(missing_docs)] pub fn data_directory(&self) -> &pe::DataDirectories {
        self.pe_header.optional_header.as_ref().map_or(
            &pe::DataDirectories::EMPTY,
            |oh| oh.data_directory(),
        )
    }

    #[allow(missing_docs)] pub fn pe_section_header(&self, idx: impl TryInto<usize>) -> Option<&pe::SectionHeader> {
        idx.try_into().ok().and_then(|idx| self.pe_section_headers.get(idx))
    }
}



enum Src {
    Unknown,
    PathBuf(PathBuf),
}

impl Src {
    pub fn anno<T>(&self, r: io::Result<T>, note: &str) -> io::Result<T> {
        if let Err(e) = r {
            Err(io::Error::new(e.kind(), format!("{}: {}: {}", self, note, e)))
        } else {
            r
        }
    }
}

impl Default for Src {
    fn default() -> Self {
        Src::Unknown
    }
}

impl Display for Src {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Src::Unknown    => write!(fmt, "unknown"),
            Src::PathBuf(p) => write!(fmt, "`{}`", p.display()),
        }
    }
}



/// impl {[`Read`], [`Seek`], [`ReadAt`]} in terms of [`pe::Reader`]'s [`pe::SectionHeader`]s<br>
/// <br>
pub struct RvaReader<'r, R> {
    reader:             &'r Reader<R>,
    rva:                u64,
    section_idx:        usize,
    section_remaining:  u64,
}

impl<'r, R> Clone for RvaReader<'r, R> { // always cloneable even if R isn't
    fn clone(&self) -> Self {
        Self { reader: self.reader, rva: self.rva, section_idx: self.section_idx, section_remaining: self.section_remaining }
    }
}

impl<'r, R: ReadAt> RvaReader<'r, R> {
    /// Create an [`RvaReader`], starting at a seek point of `rva`.
    pub fn new(reader: &'r Reader<R>, rva: RVA) -> Self {
        let mut rr = Self {
            reader,
            rva:                rva.to_u64(),
            section_idx:        !0,
            section_remaining:  0,
        };
        rr.find_section();
        rr
    }

    fn find_section(&mut self) {
        match u32::try_from(self.rva) {
            Err(_) => {
                self.section_idx = !0;
                self.section_remaining = u64::MAX - 0x1_0000_0000 - self.rva;
            },
            Ok(rva) => {
                for (i, section) in self.reader.pe_section_headers().iter().enumerate() {
                    let rva = RVA::new(rva);
                    if section.virtual_address_range().contains(&rva) {
                        self.section_idx        = i;
                        self.section_remaining  = u64::from(rva - section.virtual_address) + u64::from(section.virtual_size);
                        return;
                    }
                }

                self.section_idx = !0;
                self.section_remaining = self.reader.pe_section_headers().iter()
                    .map(|s| s.virtual_address)
                    .filter_map(|s_rva| s_rva.to_u32().checked_sub(rva))
                    .filter(|rem| *rem > 0)
                    .min()
                    .map(u64::from)
                    .unwrap_or(u64::MAX - 0x1_0000_0000 - self.rva);
            },
        }
    }
}

impl<'r, R: ReadAt> Read for RvaReader<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let rem = usize::try_from(self.section_remaining).unwrap_or(!0);
        let to_read = buf.len().min(rem);

        let did_read = if let Some(section) = self.reader.pe_section_headers.get(self.section_idx) {
            let section_start = u64::from(section.pointer_to_raw_data.map_or(0, |nz| u32::from(nz)));

            let did_read = self.reader.src.anno(self.reader.reader.read_at(
                &mut buf[..to_read],
                section_start + u64::from(RVA::new(self.rva as u32) - section.virtual_address),
            ), "error reading from RVA")?;

            debug_assert!(did_read <= to_read);
            did_read
        } else {
            buf[..to_read].fill(0u8);
            to_read
        };

        self.rva += did_read as u64;
        self.section_remaining -= did_read as u64;
        if self.section_remaining == 0 { self.find_section() }
        Ok(did_read)
    }
}

impl<'r, R: ReadAt> Seek for RvaReader<'r, R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Current(delta) if delta < 0 => {
                self.rva = self.rva.checked_sub(delta.unsigned_abs()).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "attempted to seek before 0"))?;
            },
            SeekFrom::Current(delta) => {
                self.rva = self.rva.checked_add(delta as u64).ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "attempted to seek past u64::MAX"))?;
            },
            SeekFrom::Start(rva) => {
                self.rva = rva;
            },
            SeekFrom::End(delta) if delta <= 0 => {
                self.rva = u64::MAX - delta.unsigned_abs();
            },
            SeekFrom::End(_) => {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "cannot seek past end of RVA address range"));
            },
        }
        self.find_section();
        Ok(self.rva)
    }
}

impl<'r, R: ReadAt> maulingmonkey_io_adapters::ReadAt for RvaReader<'r, R> {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let mut r = (*self).clone();
        r.seek(SeekFrom::Start(offset))?;
        r.read(buf)
    }
}

use crate::*;

use std::cell::RefCell;
use std::convert::*;
use std::fmt::{self, *};
use std::fs::File;
use std::io::{self, *};
use std::ops::*;
use std::path::PathBuf;



pub struct Reader<R> {
    src:                        Src,
    reader:                     RefCell<R>,
    exe_start:                  u64,
    mz_header:                  mz::Header,
    pe_header:                  pe::Header,
    pe_section_headers:         Vec<pe::SectionHeader>,
}

impl Reader<io::BufReader<File>> {
    pub fn open(path: impl Into<PathBuf>) -> io::Result<Reader<io::BufReader<File>>> {
        let path = path.into();
        let file = File::open(&path);
        let src = Src::PathBuf(path);
        Reader::read_src(BufReader::new(src.anno(file, "error opening pe::Reader")?), src)
    }
}

impl<R: Read + Seek> Reader<R> {
    pub fn read(reader: R) -> io::Result<Self> { Self::read_src(reader, Src::Unknown) }

    pub fn get_pe_section_header(&self, idx: impl TryInto<u16>) -> Option<&pe::SectionHeader> {
        let idx = idx.try_into().ok()?;
        self.pe_section_headers.get(usize::from(idx))
    }

    pub fn pe_section_header(&self, idx: impl TryInto<u16>) -> &pe::SectionHeader {
        self.get_pe_section_header(idx).expect("pe::Reader::pe_section_header(idx): idx out of bounds")
    }

    pub fn read_pe_section_data_by_idx<I>(&self, idx: I) -> io::Result<Vec<u8>> where I : TryInto<u16> {
        let header = *self.pe_section_header(idx);
        self.read_pe_section_data(&header)
    }

    pub fn read_pe_section_data(&self, header: &pe::SectionHeader) -> io::Result<Vec<u8>> {
        let mut data = vec![0u8; usize::try_from(header.size_of_raw_data).expect("unable to allocate pe::SectionHeader::size_of_raw_data bytes")];
        self.read_pe_section_data_inplace(header, 0, &mut data)?;
        Ok(data)
    }

    pub fn read_pe_section_data_inplace<'a>(&'_ self, header: &'_ pe::SectionHeader, offset: u32, data: &'a mut [u8]) -> io::Result<&'a [u8]> {
        let ptr = u64::from(header.pointer_to_raw_data.map_or(0, |ptr| ptr.into())) + u64::from(offset);
        let n = usize::try_from(header.size_of_raw_data.saturating_sub(offset)).unwrap_or(!0).min(data.len());
        if n > 0 {
            self.seek_to(self.exe_start, ptr, "error seeking to PE section")?;
            self.src.anno(self.reader.borrow_mut().read_exact(&mut data[..n]), "error reading PE section")?;
        }
        Ok(&data[..n])
    }

    pub fn pe_section_headers(&self) -> &[pe::SectionHeader] { &self.pe_section_headers[..] }

    pub fn read_exact_rva<'a>(&'_ self, rva: Range<u32>, scratch: &'a mut Vec<u8>) -> io::Result<&'a [u8]> {
        let size = (rva.end - rva.start) as usize;
        if scratch.len() < size { scratch.resize(size, 0u8); }

        let mut rva = rva.start;
        let mut o = &mut scratch[..size];
        while !o.is_empty() {
            if let Some(section) = self.pe_section_headers.iter().find(|s| s.virtual_address_range().contains(&rva)).copied() {
                let n = self.read_pe_section_data_inplace(&section, rva - section.virtual_address, o)?.len();
                rva += n as u32;
                o = &mut o[n..];
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "not all RVA mapped")); // XXX
            }
        }

        Ok(&scratch[..size])
    }

    fn read_src(mut reader: R, src: Src) -> io::Result<Self> {
        let exe_start = src.anno(reader.stream_position(), "error reading stream position for mz::Header")?;
        let mz_header = src.anno(mz::Header::read_from(&mut reader), "error reading mz::Header")?;
        src.anno(reader.seek(SeekFrom::Start(mz_header.pe_header_start.into())), "error seeking to pe::Header")?;
        let pe_header = src.anno(pe::Header::read_from(&mut reader), "error reading pe::Header")?;

        let mut pe_section_headers = vec![pe::SectionHeader::default(); pe_header.file_header.nsections.into()];
        for section in &mut pe_section_headers {
            *section = src.anno(pe::SectionHeader::read_from(&mut reader), "error reading pe::SectionHeader")?;
        }

        let reader = RefCell::new(reader);

        Ok(Self {
            src,
            reader,
            exe_start,
            mz_header,
            pe_header,
            pe_section_headers,
        })
    }

    fn seek_to(&self, start: u64, offset: impl Into<u64>, anno: &str) -> io::Result<u64> {
        self.src.anno(self.reader.borrow_mut().seek(SeekFrom::Start(start + offset.into())), anno)
    }
}

impl<R> Reader<R> {
    pub fn mz_header(&self) -> &mz::Header { &self.mz_header }
    pub fn pe_header(&self) -> &pe::Header { &self.pe_header }

    pub fn data_directory(&self) -> &pe::DataDirectories {
        self.pe_header.optional_header.as_ref().map_or(
            &pe::DataDirectories::EMPTY,
            |oh| oh.data_directory(),
        )
    }
}

#[test] fn self_test() {
    let _exe = Reader::open(std::env::args_os().next().expect("argv[0] not available")).expect("unable to read exe");
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

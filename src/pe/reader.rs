use crate::*;

use std::convert::*;
use std::fmt::{self, *};
use std::fs::File;
use std::io::{self, *};
use std::mem::*;
use std::path::PathBuf;



#[cfg(    debug_assertions )] const FIXED_SECTION_HEADERS : usize = 1;
#[cfg(not(debug_assertions))] const FIXED_SECTION_HEADERS : usize = 32;

pub struct Reader<R> {
    src:                        Src,
    reader:                     R,
    exe_start:                  u64,
    mz_header:                  mz::Header,
    pe_header:                  pe::Header,
    pe_section_headers_start:   u64,
    pe_section_headers_cache:   [pe::SectionHeader; FIXED_SECTION_HEADERS],
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

    pub fn read_pe_section_header<I>(&mut self, idx: I) -> io::Result<pe::SectionHeader> where I : TryInto<u16>, I::Error : Debug {
        let idx = idx.try_into().expect("pe::Reader::pe_section_header: idx out of bounds");
        assert!(idx < self.pe_header.file_header.nsections, "pe::Reader::pe_section_header: idx out of bounds");

        if let Some(sh) = self.pe_section_headers_cache.get(usize::from(idx)) {
            Ok(*sh)
        } else {
            let sh_offset = self.pe_section_headers_start + (size_of::<pe::SectionHeader>() as u64) * u64::from(idx);
            self.src.anno(self.reader.seek(SeekFrom::Start(sh_offset)), "error seeking to pe::SectionHeader")?;
            self.src.anno(pe::SectionHeader::read_from(&mut self.reader), "error reading pe::SectionHeader")
        }
    }

    pub fn read_pe_section_data_by_idx<I>(&mut self, idx: I) -> io::Result<Vec<u8>> where I : TryInto<u16>, I::Error : Debug {
        let header = self.read_pe_section_header(idx)?;
        self.read_pe_section_data(&header)
    }

    pub fn read_pe_section_data(&mut self, header: &pe::SectionHeader) -> io::Result<Vec<u8>> {
        let ptr = u64::from(header.pointer_to_raw_data.map_or(0, |ptr| ptr.into()));
        let mut data = vec![0u8; usize::try_from(header.size_of_raw_data).expect("unable to allocate pe::SectionHeader::size_of_raw_data bytes")];
        if !data.is_empty() {
            self.src.anno(self.reader.seek(SeekFrom::Start(self.exe_start + ptr)), "error seeking to PE section")?;
            self.src.anno(self.reader.read_exact(&mut data[..]), "error reading PE section")?;
        }
        Ok(data)
    }

    pub fn read_pe_section_headers(&mut self) -> io::Result<Vec<pe::SectionHeader>> {
        (0 .. self.pe_header.file_header.nsections).map(|i| self.read_pe_section_header(i)).collect()
    }

    fn read_src(mut reader: R, src: Src) -> io::Result<Self> {
        let exe_start = src.anno(reader.stream_position(), "error reading stream position for mz::Header")?;
        let mz_header = src.anno(mz::Header::read_from(&mut reader), "error reading mz::Header")?;
        src.anno(reader.seek(SeekFrom::Start(mz_header.pe_header_start.into())), "error seeking to pe::Header")?;
        let pe_header = src.anno(pe::Header::read_from(&mut reader), "error reading pe::Header")?;

        let pe_section_headers_start = src.anno(reader.stream_position(), "error reading stream position for pe::SectionHeader[0]")?;
        let mut pe_section_headers_cache = [pe::SectionHeader::default(); FIXED_SECTION_HEADERS];
        let sections = &mut pe_section_headers_cache[..FIXED_SECTION_HEADERS.min(pe_header.file_header.nsections.into())];
        for section in sections {
            *section = src.anno(pe::SectionHeader::read_from(&mut reader), "error reading pe::SectionHeader")?;
        }

        Ok(Self {
            src,
            reader,
            exe_start,
            mz_header,
            pe_header,
            pe_section_headers_start,
            pe_section_headers_cache,
        })
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

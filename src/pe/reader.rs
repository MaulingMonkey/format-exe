use crate::*;
use super::*;

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

    pub fn read_exact_rva<'a>(&'_ self, rva: Range<RVA>, scratch: &'a mut Vec<u8>) -> io::Result<&'a [u8]> {
        let size = rva.end.to_usize() - rva.start.to_usize();
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



pub struct RvaReader<'r, R> {
    reader:             &'r mut Reader<R>,
    rva:                u64,
    section_idx:        usize,
    section_remaining:  u64,
}

impl<'r, R: Read + Seek> RvaReader<'r, R> {
    pub fn new(reader: &'r mut Reader<R>, rva: RVA) -> Self {
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

impl<'r, R: Read + Seek> Read for RvaReader<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let rem = usize::try_from(self.section_remaining).unwrap_or(!0);
        let to_read = buf.len().min(rem);

        let did_read = if let Some(section) = self.reader.pe_section_headers.get(self.section_idx) {
            let section_start = section.pointer_to_raw_data.map_or(0, |nz| u32::from(nz));
            self.reader.seek_to(self.reader.exe_start, section_start + (RVA::new(self.rva as u32) - section.virtual_address), "error seeking to RVA")?;
            let did_read = self.reader.src.anno(self.reader.reader.borrow_mut().read(&mut buf[..to_read]), "error reading from RVA")?;
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

impl<'r, R: Read + Seek> Seek for RvaReader<'r, R> {
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

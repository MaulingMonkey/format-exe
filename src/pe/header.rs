use crate::*;
use super::*;

use bytemuck::*;

use std::io::{self, *};
use std::mem::{size_of, size_of_val};



/// ⨯ {
///     [signature](Self::signature),
///     [file_header](Self::file_header),
///     [optional_header](Self::optional_header)
/// }<br>
/// The main [`pe`] header, found at [`mz::Header::pe_header_start`].<br>
/// <br>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Header {
    /// `b"PE\0\0"` for valid PE modules.
    pub signature:          Signature,
    /// Basic metadata like [`Machine`] type etc.
    pub file_header:        FileHeader,
    /// Lots more data like image base address, windows subsystem, ...
    pub optional_header:    Option<OptionalHeader>,
}

impl Header {
    /// Reads an [`pe::Header`] from `read`
    ///
    /// ## Errors
    /// * [`io::ErrorKind::InvalidData`] if [`signature`](#structfield.signature) ≠ `"PE\0\0"`
    /// * [`io::ErrorKind::UnexpectedEof`] if `read` didn't contain enough data
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        let mut header = RawHeader::default();
        read.read_exact(bytes_of_mut(&mut header))?;
        let RawHeader { signature, file_header } = header;
        if signature.buffer() != b"PE\0\0" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "pe::Header::signature != \"PE\\0\\0\""));
        }
        let file_header = FileHeader::from_raw(file_header)?;

        match file_header.optional_header_size {
            0 => return Ok(Self { signature, file_header, optional_header: None }),
            1 => return Err(io::Error::new(io::ErrorKind::InvalidData, "pe::FileHeader::optional_header_size == 1 (invalid - not big enough for magic discriminant)")),
            n => {
                let optional_header_size = usize::from(n);

                let mut magic = u16le::new(0);
                read.read_exact(bytes_of_mut(&mut magic))?;

                const IMAGE_NT_OPTIONAL_HDR32_MAGIC : u16le = u16le::new(0x010b);
                const IMAGE_NT_OPTIONAL_HDR64_MAGIC : u16le = u16le::new(0x020b);
                const IMAGE_ROM_OPTIONAL_HDR_MAGIC  : u16le = u16le::new(0x0107);

                match magic {
                    IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                        type Raw = <OptionalHeader32 as FromMemory>::Raw;
                        let mut o = Raw { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<DataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader32 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        let h = <OptionalHeader32 as FromMemory>::from_raw(o)?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader32(h)) })
                    },
                    IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                        type Raw = <OptionalHeader64 as FromMemory>::Raw;
                        let mut o = Raw { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<DataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader64 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        let h = <OptionalHeader64 as FromMemory>::from_raw(o)?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader64(h)) })
                    },
                    IMAGE_ROM_OPTIONAL_HDR_MAGIC => {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "pe::OptionalHeader::magic == IMAGE_ROM_OPTIONAL_HDR_MAGIC (unsupported value)"))
                    },
                    other => {
                        Err(io::Error::new(io::ErrorKind::InvalidData, format!("pe::OptionalHeader::magic == 0x{:04x} (unsupported value)", other.to_le())))
                    },
                }
            },
        }
    }
}



/// Similarish to [IMAGE_NT_HEADERS32](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32),
/// but without the [`machine`](#structfield.machine)-dependent optional header
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
struct RawHeader {
    /// `b"PE\0\0"` for valid PE modules.
    pub signature:                  Signature,
    /// Basic metadata like [`Machine`] type etc.
    pub file_header:                <FileHeader as FromMemory>::Raw,
}

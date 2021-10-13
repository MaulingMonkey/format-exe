//! The Windows 95+ **P**ortable **E**xecutable file format.
//!
//! ## References
//! *   <https://wiki.osdev.org/PE>

mod data_directory;                 pub use data_directory::*;
mod file_header;                    pub use file_header::*;
mod machine;                        pub use machine::*;
mod optional_header_32;             pub use optional_header_32::*;
mod optional_header_64;             pub use optional_header_64::*;
mod optional_header;                pub use optional_header::*;

use crate::*;

use bytemuck::*;

use std::io::{self, *};
use std::mem::{size_of, size_of_val};


pub type Signature = abistr::CStrBuf<[u8; 4]>;




#[derive(Clone, Copy, Debug, Default)]
pub struct Header {
    pub signature:          Signature,
    pub file_header:        FileHeader,
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
        let file_header = FileHeader::from(file_header);

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
                        let mut o = RawOptionalHeader32 { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<RawDataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader32 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader32(o.into())) })
                    },
                    IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                        let mut o = RawOptionalHeader64 { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<RawDataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader64 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader64(o.into())) })
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



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DataDirectories {
    /// IMAGE_DIRECTORY_ENTRY_EXPORT
    pub export:             DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IMPORT
    pub import:             DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_RESOURCE
    pub resource:           DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_EXCEPTION
    pub exception:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_SECURITY
    /// Certificates related stuff
    pub security:           DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BASERELOC
    /// Base relocation table
    pub basereloc:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DEBUG
    pub debug:              DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_ARCHITECTURE
    /// Architecture-specific data
    ///
    /// IMAGE_DIRECTORY_ENTRY_COPYRIGHT (x86 usage)
    pub architecture:       DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_GLOBALPTR
    ///
    /// Global pointer register relative virtual address
    pub globalptr:          DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_TLS
    /// Thread local storage (TLS)
    pub tls:                DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG
    pub load_config:        DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT
    pub bound_imports:      DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IAT
    pub iat:                DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT
    pub delay_import:       DataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR
    /// COM runtime descriptor / CLR header
    pub com_descriptor:     DataDirectory,

    _reserved:              DataDirectory,
}

impl From<RawDataDirectories> for DataDirectories {
    fn from(value: RawDataDirectories) -> Self {
        Self {
            export:                 value.export.into(),
            import:                 value.import.into(),
            resource:               value.resource.into(),
            exception:              value.exception.into(),
            security:               value.security.into(),
            basereloc:              value.basereloc.into(),
            debug:                  value.debug.into(),
            architecture:           value.architecture.into(),
            globalptr:              value.globalptr.into(),
            tls:                    value.tls.into(),
            load_config:            value.load_config.into(),
            bound_imports:          value.bound_imports.into(),
            iat:                    value.iat.into(),
            delay_import:           value.delay_import.into(),
            com_descriptor:         value.com_descriptor.into(),
            _reserved:              value._reserved.into(),
        }
    }
}



/// Similarish to [IMAGE_NT_HEADERS32](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32),
/// but without the [`machine`](#structfield.machine)-dependent optional header
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawHeader {
    pub signature:                  Signature,
    pub file_header:                RawFileHeader,
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawNtHeaders32 {
    pub signature:                  Signature,
    pub file_header:                RawFileHeader,
    pub optional_header:            RawOptionalHeader32,
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win64/api/winnt/ns-winnt-image_nt_headers64>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawNtHeaders64 {
    pub signature:                  Signature,
    pub file_header:                RawFileHeader,
    pub optional_header:            RawOptionalHeader64,
}



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawDataDirectories {
    /// IMAGE_DIRECTORY_ENTRY_EXPORT
    pub export:             RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IMPORT
    pub import:             RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_RESOURCE
    pub resource:           RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_EXCEPTION
    pub exception:          RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_SECURITY
    /// Certificates related stuff
    pub security:           RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BASERELOC
    /// Base relocation table
    pub basereloc:          RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DEBUG
    pub debug:              RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_ARCHITECTURE
    /// Architecture-specific data
    ///
    /// IMAGE_DIRECTORY_ENTRY_COPYRIGHT (x86 usage)
    pub architecture:       RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_GLOBALPTR
    ///
    /// Global pointer register relative virtual address
    pub globalptr:          RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_TLS
    /// Thread local storage (TLS)
    pub tls:                RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG
    pub load_config:        RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT
    pub bound_imports:      RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_IAT
    pub iat:                RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT
    pub delay_import:       RawDataDirectory,

    /// IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR
    /// COM runtime descriptor / CLR header
    pub com_descriptor:     RawDataDirectory,

    pub(super) _reserved:   RawDataDirectory,
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawSectionHeader {
    pub name:                       abistr::CStrBuf<[u8; 8]>,
    pub virtual_size:               u32le,
    pub virtual_address:            u32le,
    pub size_of_raw_data:           u32le,
    pub pointer_to_raw_data:        u32le,
    pub pointer_to_relocations:     u32le,
    pub pointer_to_linenumbers:     u32le,
    pub number_of_relocations:      u16le,
    pub number_of_linenumbers:      u16le,
    pub characteristics:            u32le,
}

#[test] fn layout() {
    use std::mem::*;

    const IMAGE_NUMBEROF_DIRECTORY_ENTRIES : usize = 16;
    assert_eq!(size_of::<RawDataDirectories>(), size_of::<[RawDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
    assert_eq!(align_of::<RawDataDirectories>(), align_of::<[RawDataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
}

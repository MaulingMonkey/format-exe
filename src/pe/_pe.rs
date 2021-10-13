//! The Windows 95+ **P**ortable **E**xecutable file format.
//!
//! ## References
//! *   <https://wiki.osdev.org/PE>

mod file_header;                    pub use file_header::*;
mod machine;                        pub use machine::*;

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
        let mut header = raw::Header::default();
        read.read_exact(bytes_of_mut(&mut header))?;
        let raw::Header { signature, file_header } = header;
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
                        let mut o = OptionalHeader32 { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<raw::DataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader32 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader32(o)) })
                    },
                    IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                        let mut o = OptionalHeader64 { magic, .. Default::default() };
                        let required = size_of_val(&o) - size_of::<raw::DataDirectories>();
                        if optional_header_size < required {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
                                "pe::FileHeader::optional_header_size ({}) < required size for pe::OptionalHeader64 ({})",
                                optional_header_size, required,
                            )));
                        }
                        read.read_exact(&mut bytes_of_mut(&mut o)[2..])?;
                        Ok(Self { signature, file_header, optional_header: Some(OptionalHeader::OptionalHeader64(o)) })
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
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub struct OptionalHeader32 {
    pub magic:                      u16le,
    pub linker_version:             [u8; 2],
    pub size_of_code:               u32,
    pub size_of_initialized_data:   u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point:     u32,
    pub base_of_code:               u32,
    pub base_of_data:               u32,
    pub image_base:                 u32,
    pub section_alignment:          u32,
    pub file_alignment:             u32,
    pub operating_system_version:   [u16; 2],
    pub image_version:              [u16; 2],
    pub subsystem_version:          [u16; 2],
    pub win32_version:              u32,
    pub size_of_image:              u32,
    pub size_of_headers:            u32,
    pub checksum:                   u32,
    pub subsystem:                  u16,
    pub dll_characteristics:        u16,
    pub size_of_stack_reserve:      u32,
    pub size_of_stack_commit:       u32,
    pub size_of_heap_reserve:       u32,
    pub size_of_heap_commit:        u32,
    pub loader_flags:               u32,
    pub number_of_rva_and_sizes:    u32,
    pub data_directory:             DataDirectories,
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub struct OptionalHeader64 {
    pub magic:                      u16le,
    pub linker_version:             [u8; 2],
    pub size_of_code:               u32,
    pub size_of_initialized_data:   u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point:     u32,
    pub base_of_code:               u32,
    // no base_of_data field
    pub image_base:                 u64, // 64!
    pub section_alignment:          u32,
    pub file_alignment:             u32,
    pub operating_system_version:   [u16; 2],
    pub image_version:              [u16; 2],
    pub subsystem_version:          [u16; 2],
    pub win32_version:              u32,
    pub size_of_image:              u32,
    pub size_of_headers:            u32,
    pub checksum:                   u32,
    pub subsystem:                  u16,
    pub dll_characteristics:        u16,
    pub size_of_stack_reserve:      u64, // 64!
    pub size_of_stack_commit:       u64, // 64!
    pub size_of_heap_reserve:       u64, // 64!
    pub size_of_heap_commit:        u64, // 64!
    pub loader_flags:               u32,
    pub number_of_rva_and_sizes:    u32,
    pub data_directory:             DataDirectories,
}

#[derive(Clone, Copy, Debug)]
pub enum OptionalHeader {
    OptionalHeader32(OptionalHeader32),
    OptionalHeader64(OptionalHeader64),
}

impl From<raw::OptionalHeader32> for OptionalHeader32 {
    fn from(value: raw::OptionalHeader32) -> Self {
        Self {
            magic:                      value.magic,
            linker_version:             [value.linker_version[0].into(), value.linker_version[1].into()],
            size_of_code:               value.size_of_code.into(),
            size_of_initialized_data:   value.size_of_initialized_data.into(),
            size_of_uninitialized_data: value.size_of_uninitialized_data.into(),
            address_of_entry_point:     value.address_of_entry_point.into(),
            base_of_code:               value.base_of_code.into(),
            base_of_data:               value.base_of_data.into(),
            image_base:                 value.image_base.into(),
            section_alignment:          value.section_alignment.into(),
            file_alignment:             value.file_alignment.into(),
            operating_system_version:   [value.operating_system_version[0].into(), value.operating_system_version[1].into()],
            image_version:              [value.image_version[0].into(), value.image_version[1].into()],
            subsystem_version:          [value.subsystem_version[0].into(), value.subsystem_version[1].into()],
            win32_version:              value.win32_version.into(),
            size_of_image:              value.size_of_image.into(),
            size_of_headers:            value.size_of_headers.into(),
            checksum:                   value.checksum.into(),
            subsystem:                  value.subsystem.into(),
            dll_characteristics:        value.dll_characteristics.into(),
            size_of_stack_reserve:      value.size_of_stack_reserve.into(),
            size_of_stack_commit:       value.size_of_stack_commit.into(),
            size_of_heap_reserve:       value.size_of_heap_reserve.into(),
            size_of_heap_commit:        value.size_of_heap_commit.into(),
            loader_flags:               value.loader_flags.into(),
            number_of_rva_and_sizes:    value.number_of_rva_and_sizes.into(),
            data_directory:             value.data_directory.into(),
        }
    }
}

impl From<raw::OptionalHeader64> for OptionalHeader64 {
    fn from(value: raw::OptionalHeader64) -> Self {
        Self {
            magic:                      value.magic,
            linker_version:             [value.linker_version[0].into(), value.linker_version[1].into()],
            size_of_code:               value.size_of_code.into(),
            size_of_initialized_data:   value.size_of_initialized_data.into(),
            size_of_uninitialized_data: value.size_of_uninitialized_data.into(),
            address_of_entry_point:     value.address_of_entry_point.into(),
            base_of_code:               value.base_of_code.into(),
            // no base_of_data field
            image_base:                 value.image_base.into(),
            section_alignment:          value.section_alignment.into(),
            file_alignment:             value.file_alignment.into(),
            operating_system_version:   [value.operating_system_version[0].into(), value.operating_system_version[1].into()],
            image_version:              [value.image_version[0].into(), value.image_version[1].into()],
            subsystem_version:          [value.subsystem_version[0].into(), value.subsystem_version[1].into()],
            win32_version:              value.win32_version.into(),
            size_of_image:              value.size_of_image.into(),
            size_of_headers:            value.size_of_headers.into(),
            checksum:                   value.checksum.into(),
            subsystem:                  value.subsystem.into(),
            dll_characteristics:        value.dll_characteristics.into(),
            size_of_stack_reserve:      value.size_of_stack_reserve.into(),
            size_of_stack_commit:       value.size_of_stack_commit.into(),
            size_of_heap_reserve:       value.size_of_heap_reserve.into(),
            size_of_heap_commit:        value.size_of_heap_commit.into(),
            loader_flags:               value.loader_flags.into(),
            number_of_rva_and_sizes:    value.number_of_rva_and_sizes.into(),
            data_directory:             value.data_directory.into(),
        }
    }
}


/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
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

impl From<raw::DataDirectories> for DataDirectories {
    fn from(value: raw::DataDirectories) -> Self {
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

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub struct DataDirectory {
    pub virtual_address:            u32,
    pub size:                       u32,
}

impl From<raw::DataDirectory> for DataDirectory {
    fn from(value: raw::DataDirectory) -> Self {
        Self {
            virtual_address:    value.virtual_address.into(),
            size:               value.size.into(),
        }
    }
}



mod raw {
    use super::*;

    /// Similarish to [IMAGE_NT_HEADERS32](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32),
    /// but without the [`machine`](#structfield.machine)-dependent optional header
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct Header {
        pub signature:                  Signature,
        pub file_header:                RawFileHeader,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct NtHeaders32 {
        pub signature:                  Signature,
        pub file_header:                RawFileHeader,
        pub optional_header:            OptionalHeader32,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win64/api/winnt/ns-winnt-image_nt_headers64>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct NtHeaders64 {
        pub signature:                  Signature,
        pub file_header:                RawFileHeader,
        pub optional_header:            OptionalHeader64,
    }


    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct OptionalHeader32 {
        pub magic:                      u16le,
        pub linker_version:             [u8le; 2],
        pub size_of_code:               u32le,
        pub size_of_initialized_data:   u32le,
        pub size_of_uninitialized_data: u32le,
        pub address_of_entry_point:     u32le,
        pub base_of_code:               u32le,
        pub base_of_data:               u32le,
        pub image_base:                 u32le,
        pub section_alignment:          u32le,
        pub file_alignment:             u32le,
        pub operating_system_version:   [u16le; 2],
        pub image_version:              [u16le; 2],
        pub subsystem_version:          [u16le; 2],
        pub win32_version:              u32le,
        pub size_of_image:              u32le,
        pub size_of_headers:            u32le,
        pub checksum:                   u32le,
        pub subsystem:                  u16le,
        pub dll_characteristics:        u16le,
        pub size_of_stack_reserve:      u32le,
        pub size_of_stack_commit:       u32le,
        pub size_of_heap_reserve:       u32le,
        pub size_of_heap_commit:        u32le,
        pub loader_flags:               u32le,
        pub number_of_rva_and_sizes:    u32le,
        pub data_directory:             DataDirectories,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct OptionalHeader64 {
        pub magic:                      u16le,
        pub linker_version:             [u8le; 2],
        pub size_of_code:               u32le,
        pub size_of_initialized_data:   u32le,
        pub size_of_uninitialized_data: u32le,
        pub address_of_entry_point:     u32le,
        pub base_of_code:               u32le,
        // no base_of_data field
        pub image_base:                 u64le, // 64!
        pub section_alignment:          u32le,
        pub file_alignment:             u32le,
        pub operating_system_version:   [u16le; 2],
        pub image_version:              [u16le; 2],
        pub subsystem_version:          [u16le; 2],
        pub win32_version:              u32le,
        pub size_of_image:              u32le,
        pub size_of_headers:            u32le,
        pub checksum:                   u32le,
        pub subsystem:                  u16le,
        pub dll_characteristics:        u16le,
        pub size_of_stack_reserve:      u64le, // 64!
        pub size_of_stack_commit:       u64le, // 64!
        pub size_of_heap_reserve:       u64le, // 64!
        pub size_of_heap_commit:        u64le, // 64!
        pub loader_flags:               u32le,
        pub number_of_rva_and_sizes:    u32le,
        pub data_directory:             DataDirectories,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory#remarks>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct DataDirectories {
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

        pub(super) _reserved:   DataDirectory,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct DataDirectory {
        pub virtual_address:            u32le,
        pub size:                       u32le,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub(crate) struct SectionHeader {
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
        assert_eq!(size_of::<DataDirectories>(), size_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
        assert_eq!(align_of::<DataDirectories>(), align_of::<[DataDirectory; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>());
    }
}

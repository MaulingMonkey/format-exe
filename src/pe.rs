//! The Windows 95+ **P**ortable **E**xecutable file format.
//!
//! ## References
//! *   <https://wiki.osdev.org/PE>

use crate::*;

use bytemuck::*;

use std::fmt::{self, *};



/// [I386](Self::I386),
/// [AMD64](Self::AMD64),
/// [IA64](Self::IA64),
/// [ARM](Self::ARM),
/// [ARM64](Self::ARM64),
/// ... &nbsp;&nbsp;&nbsp;&nbsp; Machine/instruction architectures
///
/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
/// *   `IMAGE_FILE_MACHINE_*` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)] // TODO: PartialOrd/Ord/Hash
#[derive(Pod, Zeroable)]
pub struct Machine(u16le);

impl Machine {
    pub const fn new(value: u16) -> Self {
        Self(u16le::new(value))
    }

    pub const UNKNOWN       : Machine = Machine::new(0);

    /// Useful for indicating we want to interact with the host and not a WoW guest.
    pub const TARGET_HOST   : Machine = Machine::new(0x0001);

    #[doc = "Intel 386 / x86"                       ] pub const I386          : Machine = Machine::new(0x014c);
    #[doc = "MIPS little-endian, 0x160 big-endian"  ] pub const R3000         : Machine = Machine::new(0x0162);
    #[doc = "MIPS little-endian"                    ] pub const R4000         : Machine = Machine::new(0x0166);
    #[doc = "MIPS little-endian"                    ] pub const R10000        : Machine = Machine::new(0x0168);
    #[doc = "MIPS little-endian WCE v2"             ] pub const WCEMIPSV2     : Machine = Machine::new(0x0169);
    #[doc = "Alpha_AXP"                             ] pub const ALPHA         : Machine = Machine::new(0x0184);
    #[doc = "SH3 little-endian"                     ] pub const SH3           : Machine = Machine::new(0x01a2);
    #[doc = ""                                      ] pub const SH3DSP        : Machine = Machine::new(0x01a3);
    #[doc = "SH3E little-endian"                    ] pub const SH3E          : Machine = Machine::new(0x01a4);
    #[doc = "SH4 little-endian"                     ] pub const SH4           : Machine = Machine::new(0x01a6);
    #[doc = "SH5"                                   ] pub const SH5           : Machine = Machine::new(0x01a8);
    #[doc = "ARM Little-Endian"                     ] pub const ARM           : Machine = Machine::new(0x01c0);
    #[doc = "ARM Thumb/Thumb-2 Little-Endian"       ] pub const THUMB         : Machine = Machine::new(0x01c2);
    #[doc = "ARM Thumb-2 Little-Endian"             ] pub const ARMNT         : Machine = Machine::new(0x01c4);
    #[doc = ""                                      ] pub const AM33          : Machine = Machine::new(0x01d3);
    #[doc = "IBM PowerPC Little-Endian"             ] pub const POWERPC       : Machine = Machine::new(0x01F0);
    #[doc = ""                                      ] pub const POWERPCFP     : Machine = Machine::new(0x01f1);
    #[doc = "Intel Itanium"                         ] pub const IA64          : Machine = Machine::new(0x0200);
    #[doc = "MIPS"                                  ] pub const MIPS16        : Machine = Machine::new(0x0266);
    #[doc = "ALPHA64"                               ] pub const ALPHA64       : Machine = Machine::new(0x0284);
    #[doc = "MIPS"                                  ] pub const MIPSFPU       : Machine = Machine::new(0x0366);
    #[doc = "MIPS"                                  ] pub const MIPSFPU16     : Machine = Machine::new(0x0466);
    #[doc = "ALPHA64"                               ] pub const AXP64         : Machine = Self::ALPHA64;
    #[doc = "Infineon"                              ] pub const TRICORE       : Machine = Machine::new(0x0520);
    #[doc = ""                                      ] pub const CEF           : Machine = Machine::new(0x0CEF);
    #[doc = "EFI Byte Code"                         ] pub const EBC           : Machine = Machine::new(0x0EBC);
    #[doc = "AMD64 (K8) / x64 / x86-64"             ] pub const AMD64         : Machine = Machine::new(0x8664);
    #[doc = "M32R little-endian"                    ] pub const M32R          : Machine = Machine::new(0x9041);
    #[doc = "ARM64 Little-Endian"                   ] pub const ARM64         : Machine = Machine::new(0xAA64);
    #[doc = ""                                      ] pub const CEE           : Machine = Machine::new(0xC0EE);
}

impl Debug for Machine {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #[allow(unreachable_patterns)] // contains duplicates (AXP64)
        match *self {
            Self::UNKNOWN       => write!(fmt, "Machine::UNKNOWN"),
            Self::TARGET_HOST   => write!(fmt, "Machine::TARGET_HOST"),
            Self::I386          => write!(fmt, "Machine::I386"),
            Self::R3000         => write!(fmt, "Machine::R3000"),
            Self::R4000         => write!(fmt, "Machine::R4000"),
            Self::R10000        => write!(fmt, "Machine::R10000"),
            Self::WCEMIPSV2     => write!(fmt, "Machine::WCEMIPSV2"),
            Self::ALPHA         => write!(fmt, "Machine::ALPHA"),
            Self::SH3           => write!(fmt, "Machine::SH3"),
            Self::SH3DSP        => write!(fmt, "Machine::SH3DSP"),
            Self::SH3E          => write!(fmt, "Machine::SH3E"),
            Self::SH4           => write!(fmt, "Machine::SH4"),
            Self::SH5           => write!(fmt, "Machine::SH5"),
            Self::ARM           => write!(fmt, "Machine::ARM"),
            Self::THUMB         => write!(fmt, "Machine::THUMB"),
            Self::ARMNT         => write!(fmt, "Machine::ARMNT"),
            Self::AM33          => write!(fmt, "Machine::AM33"),
            Self::POWERPC       => write!(fmt, "Machine::POWERPC"),
            Self::POWERPCFP     => write!(fmt, "Machine::POWERPCFP"),
            Self::IA64          => write!(fmt, "Machine::IA64"),
            Self::MIPS16        => write!(fmt, "Machine::MIPS16"),
            Self::ALPHA64       => write!(fmt, "Machine::ALPHA64"),
            Self::MIPSFPU       => write!(fmt, "Machine::MIPSFPU"),
            Self::MIPSFPU16     => write!(fmt, "Machine::MIPSFPU16"),
            Self::AXP64         => write!(fmt, "Machine::AXP64"),
            Self::TRICORE       => write!(fmt, "Machine::TRICORE"),
            Self::CEF           => write!(fmt, "Machine::CEF"),
            Self::EBC           => write!(fmt, "Machine::EBC"),
            Self::AMD64         => write!(fmt, "Machine::AMD64"),
            Self::M32R          => write!(fmt, "Machine::M32R"),
            Self::ARM64         => write!(fmt, "Machine::ARM64"),
            Self::CEE           => write!(fmt, "Machine::CEE"),
            other               => write!(fmt, "Machine(0x{:04x})", other.0.to_le()),
        }
    }
}



// TODO

mod raw {
    use super::*;

    /// Similarish to [IMAGE_NT_HEADERS32](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32),
    /// but without the [`machine`](#structfield.machine)-dependent optional header
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct Header {
        pub signature:                  abistr::CStrBuf<[u8; 4]>, // "PE\0\0"
        pub file_header:                FileHeader,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct FileHeader {
        pub machine:                    Machine,
        pub nsections:                  u16le,
        pub time_date_stamp:            u32le,
        pub symbols:                    u32le,
        pub nsymbols:                   u32le,
        pub optional_header_size:       u16le,
        pub characteristics:            u16le,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct OptionalHeader32 {
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
    pub struct OptionalHeader64 {
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

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct DataDirectory {
        pub virtual_address:            u32le,
        pub size:                       u32le,
    }

    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[derive(Pod, Zeroable)]
    pub struct SectionHeader {
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

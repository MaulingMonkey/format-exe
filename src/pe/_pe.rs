//! The Windows 95+ **P**ortable **E**xecutable file format.
//!
//! ## References
//! *   <https://wiki.osdev.org/PE>

mod data_directories;               pub use data_directories::*;
mod data_directory;                 pub use data_directory::*;
mod file_header;                    pub use file_header::*;
mod header;                         pub use header::*;
mod machine;                        pub use machine::*;
mod optional_header_32;             pub use optional_header_32::*;
mod optional_header_64;             pub use optional_header_64::*;
mod optional_header;                pub use optional_header::*;

use crate::*;

use bytemuck::*;


pub type Signature = abistr::CStrBuf<[u8; 4]>;



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

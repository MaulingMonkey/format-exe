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
mod section_header;                 pub use section_header::*;

use crate::*;

use bytemuck::*;

pub type Signature = abistr::CStrBuf<[u8; 4]>;

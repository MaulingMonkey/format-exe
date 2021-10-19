//! The Windows 95+ **P**ortable **E**xecutable file format.
//!
//! ## References
//! *   <https://wiki.osdev.org/PE>

mod data_directories;               pub use data_directories::*;
mod data_directory;                 pub use data_directory::*;
mod dll_characteristics;            pub use dll_characteristics::*;
mod file_characteristics;           pub use file_characteristics::*;
mod file_header;                    pub use file_header::*;
mod header;                         pub use header::*;
mod import_descriptor;              pub use import_descriptor::*;
mod import_lookup_table;            pub use import_lookup_table::*;
mod machine;                        pub use machine::*;
mod optional_header_32;             pub use optional_header_32::*;
mod optional_header_64;             pub use optional_header_64::*;
mod optional_header;                pub use optional_header::*;
mod reader;                         pub use reader::*;
mod section_characteristics;        pub use section_characteristics::*;
mod section_header;                 pub use section_header::*;
mod subsystem;                      pub use subsystem::*;
mod time_date;                      pub use time_date::*;
mod version;                        pub use version::*;

type Signature = abistr::CStrBuf<[u8; 4]>;

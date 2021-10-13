use crate::*;
use super::*;

use bytemuck::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
//#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FileHeader {
    pub machine:                    Machine,
    pub nsections:                  u16,
    pub link_time_date:             TimeDate,
    pub symbols:                    u32,
    pub nsymbols:                   u32,
    pub optional_header_size:       u16,
    pub characteristics:            u16,
}

impl From<RawFileHeader> for FileHeader {
    fn from(value: RawFileHeader) -> Self {
        Self {
            machine:                value.machine,
            nsections:              value.nsections.into(),
            link_time_date:         value.link_time_date,
            symbols:                value.symbols.into(),
            nsymbols:               value.nsymbols.into(),
            optional_header_size:   value.optional_header_size.into(),
            characteristics:        value.characteristics.into(),
        }
    }
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawFileHeader {
    pub machine:                    Machine,
    pub nsections:                  u16le,
    pub link_time_date:             TimeDate,
    pub symbols:                    u32le,
    pub nsymbols:                   u32le,
    pub optional_header_size:       u16le,
    pub characteristics:            u16le,
}

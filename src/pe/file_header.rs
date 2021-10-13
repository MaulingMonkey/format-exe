use super::*;

use bytemuck::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
//#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FileHeader {
    pub machine:                    Machine,
    pub nsections:                  u16,
    pub time_date_stamp:            u32,
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
            time_date_stamp:        value.time_date_stamp.into(),
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
    pub time_date_stamp:            u32le,
    pub symbols:                    u32le,
    pub nsymbols:                   u32le,
    pub optional_header_size:       u16le,
    pub characteristics:            u16le,
}

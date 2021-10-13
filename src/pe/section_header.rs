use crate::*;
use bytemuck::*;


/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
pub struct SectionHeader {
    pub name:                       abistr::CStrBuf<[u8; 8]>,
    pub virtual_size:               u32,
    pub virtual_address:            u32,
    pub size_of_raw_data:           u32,
    pub pointer_to_raw_data:        u32,
    pub pointer_to_relocations:     u32,
    pub pointer_to_linenumbers:     u32,
    pub number_of_relocations:      u16,
    pub number_of_linenumbers:      u16,
    pub characteristics:            u32,
}

impl From<RawSectionHeader> for SectionHeader {
    fn from(value: RawSectionHeader) -> Self {
        Self {
            name:                   value.name,
            virtual_size:           value.virtual_size.into(),
            virtual_address:        value.virtual_address.into(),
            size_of_raw_data:       value.size_of_raw_data.into(),
            pointer_to_raw_data:    value.pointer_to_raw_data.into(),
            pointer_to_relocations: value.pointer_to_relocations.into(),
            pointer_to_linenumbers: value.pointer_to_linenumbers.into(),
            number_of_relocations:  value.number_of_relocations.into(),
            number_of_linenumbers:  value.number_of_linenumbers.into(),
            characteristics:        value.characteristics.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
struct RawSectionHeader {
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

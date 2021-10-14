use crate::*;
use super::*;

use bytemuck::*;

use std::io::{self, *};
use std::num::NonZeroU32;
use std::ops::Range;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SectionHeader {
    pub name:                       abistr::CStrBuf<[u8; 8]>,
    pub virtual_size:               u32,
    pub virtual_address:            u32,
    pub size_of_raw_data:           u32,
    pub pointer_to_raw_data:        Option<NonZeroU32>,
    pub pointer_to_relocations:     Option<NonZeroU32>,
    pub pointer_to_linenumbers:     Option<NonZeroU32>,
    pub number_of_relocations:      u16,
    pub number_of_linenumbers:      u16,
    pub characteristics:            SectionCharacteristics,
}

impl SectionHeader {
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        let mut s = RawSectionHeader::default();
        read.read_exact(bytes_of_mut(&mut s))?;
        Ok(s.into())
    }

    pub fn virtual_address_range(&self) -> Range<u32> {
        self.virtual_address .. self.virtual_address + self.virtual_size
    }
}

impl From<RawSectionHeader> for SectionHeader {
    fn from(value: RawSectionHeader) -> Self {
        Self {
            name:                   value.name,
            virtual_size:           value.virtual_size.into(),
            virtual_address:        value.virtual_address.into(),
            size_of_raw_data:       value.size_of_raw_data.into(),
            pointer_to_raw_data:    NonZeroU32::new(value.pointer_to_raw_data.into()),
            pointer_to_relocations: NonZeroU32::new(value.pointer_to_relocations.into()),
            pointer_to_linenumbers: NonZeroU32::new(value.pointer_to_linenumbers.into()),
            number_of_relocations:  value.number_of_relocations.into(),
            number_of_linenumbers:  value.number_of_linenumbers.into(),
            characteristics:        SectionCharacteristics::from_bits_truncate(value.characteristics.into()),
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

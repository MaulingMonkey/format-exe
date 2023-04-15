use crate::*;
use super::*;

use bytemuck::*;

use std::io::{self, *};
use std::num::NonZeroU32;
use std::ops::Range;



from_memory_struct! {
    /// {
    ///     [name](Self::name),
    ///     [virtual_size](Self::virtual_size),
    ///     [virtual_address](Self::virtual_address),
    ///     [size_of_raw_data](Self::size_of_raw_data),
    ///     ...,
    ///     [characteristics](Self::characteristics)
    /// }<br>
    /// Binary sections (e.g. `".text"`) and how to load them (ranges, read/write permissions, etc.)<br>
    /// <br>
    ///
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_section_header>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct SectionHeader {
        pub name:                       abistr::CStrBuf<[u8; 8]>,
        pub virtual_size:               u32,
        pub virtual_address:            RVA,
        pub size_of_raw_data:           u32,
        pub pointer_to_raw_data:        Option<NonZeroU32>,
        pub pointer_to_relocations:     Option<NonZeroU32>,
        pub pointer_to_linenumbers:     Option<NonZeroU32>,
        pub number_of_relocations:      u16,
        pub number_of_linenumbers:      u16,
        pub characteristics:            SectionCharacteristics,
    }
}

impl SectionHeader {
    pub fn read_from(read: &mut impl Read) -> io::Result<Self> {
        Self::from_io(read)
    }

    pub fn virtual_address_range(&self) -> Range<RVA> {
        self.virtual_address .. self.virtual_address + self.virtual_size
    }
}

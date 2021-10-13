use crate::*;

use bytemuck::*;

use std::fmt::{self, *};



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct DataDirectory {
    pub virtual_address:            u32,
    pub size:                       u32,
}

impl From<RawDataDirectory> for DataDirectory {
    fn from(value: RawDataDirectory) -> Self {
        Self {
            virtual_address:    value.virtual_address.into(),
            size:               value.size.into(),
        }
    }
}

impl Debug for DataDirectory {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if *self == DataDirectory::default() {
            write!(fmt, "DataDirectory::default()")
        } else {
            write!(fmt, "DataDirectory {{ virtual_address: 0x{:08x}, size: {} }}", self.virtual_address, self.size)
        }
    }
}



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawDataDirectory {
    pub virtual_address:            u32le,
    pub size:                       u32le,
}

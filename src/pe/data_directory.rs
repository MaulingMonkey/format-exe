use crate::*;

use bytemuck::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawDataDirectory {
    pub virtual_address:            u32le,
    pub size:                       u32le,
}

use crate::*;
use super::*;

use bytemuck::*;



/// ## References
/// *   `IMAGE_IMPORT_DESCRIPTOR` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
#[derive(Clone, Copy, Debug, Default)]
pub struct ImportDescriptor {
    characteristics_or_original_first_thunk:    u32,
    pub time_date_stamp:                        TimeDate,
    pub forwarder_chain:                        u32, // !0 if no forwarders
    pub name:                                   u32,
    pub first_thunk:                            u32, // RVA to IAT
}

impl ImportDescriptor {
    // ...
}

impl From<RawImportDescriptor> for ImportDescriptor {
    fn from(value: RawImportDescriptor) -> Self {
        Self {
            characteristics_or_original_first_thunk:    value.characteristics_or_original_first_thunk.into(),
            time_date_stamp:                            value.time_date_stamp.into(),
            forwarder_chain:                            value.forwarder_chain.into(),
            name:                                       value.name.into(),
            first_thunk:                                value.first_thunk.into(),
        }
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
struct RawImportDescriptor {
    pub characteristics_or_original_first_thunk:    u32le,
    pub time_date_stamp:                            TimeDate,
    pub forwarder_chain:                            u32le, // !0 if no forwarders
    pub name:                                       u32le,
    pub first_thunk:                                u32le, // RVA to IAT
}

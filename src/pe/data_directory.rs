use super::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};



from_memory_struct! {
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_data_directory>
    #[repr(C)]
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    pub struct DataDirectory {
        pub virtual_address:    RVA,
        pub size:               u32,
    }
}

impl DataDirectory {
    pub const EMPTY : DataDirectory = DataDirectory { virtual_address: RVA::NULL, size: 0 };
}

impl Debug for DataDirectory {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if *self == DataDirectory::EMPTY {
            write!(fmt, "DataDirectory::EMPTY")
        } else {
            write!(fmt, "DataDirectory {{ virtual_address: {:?}, size: {} }}", self.virtual_address, self.size)
        }
    }
}

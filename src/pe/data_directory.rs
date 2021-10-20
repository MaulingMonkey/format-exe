use super::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};



from_memory_struct! {
    /// {
    ///     [virtual_address](struct.DataDirectory.html#structfield.virtual_address): [RVA],
    ///     [size](struct.DataDirectory.html#structfield.size): [u32]
    /// } &nbsp;&nbsp;&nbsp;&nbsp; Describes a memory region of the loaded executable.
    ///
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

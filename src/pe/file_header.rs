use super::*;

use bytemuck::*;



from_memory_struct! {
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct FileHeader {
        pub machine:                    Machine,
        pub nsections:                  u16,
        pub link_time_date:             TimeDate,
        pub symbols:                    u32,
        pub nsymbols:                   u32,
        pub optional_header_size:       u16,
        pub characteristics:            FileCharacteristics,
    }
}

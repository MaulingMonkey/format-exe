use crate::*;
use pe::*;

use bytemuck::*;



from_memory_struct! {
    /// {
    ///     [machine](struct.FileHeader.html#structfield.machine),
    ///     [nsections](struct.FileHeader.html#structfield.nsections),
    ///     [link_time_date](struct.FileHeader.html#structfield.link_time_date),
    ///     [symbols](struct.FileHeader.html#structfield.symbols),
    ///     ...
    /// }<br>
    /// [pe::Header::file_header]: The non-optional, pointer-width independent part of said header<br>
    /// <br>
    ///
    /// ## References
    /// *   [`Header::file_header`]
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

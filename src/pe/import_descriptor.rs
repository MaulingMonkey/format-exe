use super::*;



from_memory_struct! {
    /// ## References
    /// *   `IMAGE_IMPORT_DESCRIPTOR` in `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\winnt.h`
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct ImportDescriptor {
        pub import_lookup_table_rva:    u32,
        pub time_date_stamp:            TimeDate,
        pub forwarder_chain:            u32, // !0 if no forwarders
        pub dll_ascii_name_rva:         u32,
        pub iat_rva:                    u32,
    }
}

impl ImportDescriptor {
    // ...
}

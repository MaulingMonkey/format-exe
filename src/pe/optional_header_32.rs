use super::*;

use bytemuck::*;



from_memory_struct! {
    /// "Optional" header for 32-bit executables/DLLs.
    ///
    /// ## References
    /// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct OptionalHeader32 {
        pub magic:                      u16,
        pub linker_version:             MajorMinorVersion<u8>,
        pub size_of_code:               u32,
        pub size_of_initialized_data:   u32,
        pub size_of_uninitialized_data: u32,
        pub address_of_entry_point:     u32,
        pub base_of_code:               u32,
        pub base_of_data:               u32,
        pub image_base:                 u32,
        pub section_alignment:          u32,
        pub file_alignment:             u32,
        pub operating_system_version:   MajorMinorVersion<u16>,
        pub image_version:              MajorMinorVersion<u16>,
        pub subsystem_version:          MajorMinorVersion<u16>,
        pub win32_version:              u32,
        pub size_of_image:              u32,
        pub size_of_headers:            u32,
        pub checksum:                   u32,
        pub subsystem:                  Subsystem,
        pub dll_characteristics:        DllCharacteristics,
        pub size_of_stack_reserve:      u32,
        pub size_of_stack_commit:       u32,
        pub size_of_heap_reserve:       u32,
        pub size_of_heap_commit:        u32,
        pub loader_flags:               u32,
        pub number_of_rva_and_sizes:    u32,
        pub data_directory:             DataDirectories,
    }
}

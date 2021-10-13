use crate::*;
use super::*;

use bytemuck::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OptionalHeader32 {
    pub magic:                      u16le,
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
    pub dll_characteristics:        u16,
    pub size_of_stack_reserve:      u32,
    pub size_of_stack_commit:       u32,
    pub size_of_heap_reserve:       u32,
    pub size_of_heap_commit:        u32,
    pub loader_flags:               u32,
    pub number_of_rva_and_sizes:    u32,
    pub data_directory:             DataDirectories,
}

impl From<RawOptionalHeader32> for OptionalHeader32 {
    fn from(value: RawOptionalHeader32) -> Self {
        Self {
            magic:                      value.magic,
            linker_version:             value.linker_version.into(),
            size_of_code:               value.size_of_code.into(),
            size_of_initialized_data:   value.size_of_initialized_data.into(),
            size_of_uninitialized_data: value.size_of_uninitialized_data.into(),
            address_of_entry_point:     value.address_of_entry_point.into(),
            base_of_code:               value.base_of_code.into(),
            base_of_data:               value.base_of_data.into(),
            image_base:                 value.image_base.into(),
            section_alignment:          value.section_alignment.into(),
            file_alignment:             value.file_alignment.into(),
            operating_system_version:   value.operating_system_version.into(),
            image_version:              value.image_version.into(),
            subsystem_version:          value.subsystem_version.into(),
            win32_version:              value.win32_version.into(),
            size_of_image:              value.size_of_image.into(),
            size_of_headers:            value.size_of_headers.into(),
            checksum:                   value.checksum.into(),
            subsystem:                  value.subsystem.into(),
            dll_characteristics:        value.dll_characteristics.into(),
            size_of_stack_reserve:      value.size_of_stack_reserve.into(),
            size_of_stack_commit:       value.size_of_stack_commit.into(),
            size_of_heap_reserve:       value.size_of_heap_reserve.into(),
            size_of_heap_commit:        value.size_of_heap_commit.into(),
            loader_flags:               value.loader_flags.into(),
            number_of_rva_and_sizes:    value.number_of_rva_and_sizes.into(),
            data_directory:             value.data_directory.into(),
        }
    }
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
pub(crate) struct RawOptionalHeader32 {
    pub magic:                      u16le,
    pub linker_version:             MajorMinorVersion<u8le>,
    pub size_of_code:               u32le,
    pub size_of_initialized_data:   u32le,
    pub size_of_uninitialized_data: u32le,
    pub address_of_entry_point:     u32le,
    pub base_of_code:               u32le,
    pub base_of_data:               u32le,
    pub image_base:                 u32le,
    pub section_alignment:          u32le,
    pub file_alignment:             u32le,
    pub operating_system_version:   MajorMinorVersion<u16le>,
    pub image_version:              MajorMinorVersion<u16le>,
    pub subsystem_version:          MajorMinorVersion<u16le>,
    pub win32_version:              u32le,
    pub size_of_image:              u32le,
    pub size_of_headers:            u32le,
    pub checksum:                   u32le,
    pub subsystem:                  Subsystem,
    pub dll_characteristics:        u16le,
    pub size_of_stack_reserve:      u32le,
    pub size_of_stack_commit:       u32le,
    pub size_of_heap_reserve:       u32le,
    pub size_of_heap_commit:        u32le,
    pub loader_flags:               u32le,
    pub number_of_rva_and_sizes:    u32le,
    pub data_directory:             RawDataDirectories,
}

use super::*;



/// ∑ [OptionalHeader32] | [OptionalHeader64]
#[derive(Clone, Copy)]
pub enum OptionalHeader {
    OptionalHeader32(OptionalHeader32),
    OptionalHeader64(OptionalHeader64),
}

impl core::fmt::Debug for OptionalHeader {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::OptionalHeader32(oh) => core::fmt::Debug::fmt(oh, fmt),
            Self::OptionalHeader64(oh) => core::fmt::Debug::fmt(oh, fmt),
        }
    }
}

impl OptionalHeader {
    pub fn dll_characteristics(&self) -> DllCharacteristics {
        match self {
            Self::OptionalHeader32(oh) => oh.dll_characteristics,
            Self::OptionalHeader64(oh) => oh.dll_characteristics,
        }
    }

    pub fn data_directory(&self) -> &DataDirectories {
        match self {
            Self::OptionalHeader32(oh) => &oh.data_directory,
            Self::OptionalHeader64(oh) => &oh.data_directory,
        }
    }
}

impl From<OptionalHeader32> for OptionalHeader {
    fn from(header: OptionalHeader32) -> Self {
        Self::OptionalHeader32(header)
    }
}

impl From<OptionalHeader64> for OptionalHeader {
    fn from(header: OptionalHeader64) -> Self {
        Self::OptionalHeader64(header)
    }
}

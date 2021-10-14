use super::*;



#[derive(Clone, Copy, Debug)]
pub enum OptionalHeader {
    OptionalHeader32(OptionalHeader32),
    OptionalHeader64(OptionalHeader64),
}

impl OptionalHeader {
    pub fn data_directory(&self) -> &DataDirectories {
        match self {
            Self::OptionalHeader32(oh) => &oh.data_directory,
            Self::OptionalHeader64(oh) => &oh.data_directory,
        }
    }
}

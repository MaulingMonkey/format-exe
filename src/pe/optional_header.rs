use super::*;



#[derive(Clone, Copy, Debug)]
pub enum OptionalHeader {
    OptionalHeader32(OptionalHeader32),
    OptionalHeader64(OptionalHeader64),
}

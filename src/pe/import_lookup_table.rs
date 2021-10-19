use crate::*;



/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-lookup-table>
pub trait ImportLookupTableEntry {
    /// `true` if `self.0` == `0` (end of the table
    fn is_eot           (&self) -> bool;
    fn import_by_ordinal(&self) -> bool;
    fn import_by_name   (&self) -> bool;
    fn ordinal          (&self) -> Option<u16>;
    fn name_table_rva   (&self) -> Option<u32>;
}

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-lookup-table>
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)] // TODO: PartialOrd, Ord, Hash
pub struct ImportLookupTableEntry32(u32le);

/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-lookup-table>
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)] // TODO: PartialOrd, Ord, Hash
pub struct ImportLookupTableEntry64(u64le);



impl FromMemory for ImportLookupTableEntry32 {
    type Raw    = u32le;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Self(raw)) }
}

impl FromMemory for ImportLookupTableEntry64 {
    type Raw    = u64le;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Self(raw)) }
}

impl ImportLookupTableEntry for ImportLookupTableEntry32 {
    fn is_eot           (&self) -> bool { self.0 == u32le::new(0) }
    fn import_by_ordinal(&self) -> bool { (self.0.to_le() & (1 << 31)) != 0 }
    fn import_by_name   (&self) -> bool { (self.0.to_le() & (1 << 31)) != 0 }

    fn ordinal(&self) -> Option<u16> {
        if self.import_by_ordinal() {
            Some(self.0.to_le() as u16)
        } else {
            None
        }
    }

    fn name_table_rva(&self) -> Option<u32> {
        if self.import_by_name() {
            Some(self.0.to_le() as u32 & 0x7FFF_FFFF)
        } else {
            None
        }
    }
}

impl ImportLookupTableEntry for ImportLookupTableEntry64 {
    fn is_eot           (&self) -> bool { self.0 == u64le::new(0) }
    fn import_by_ordinal(&self) -> bool { (self.0.to_le() & (1 << 63)) != 0 }
    fn import_by_name   (&self) -> bool { (self.0.to_le() & (1 << 63)) != 0 }

    fn ordinal(&self) -> Option<u16> {
        if self.import_by_ordinal() {
            Some(self.0.to_le() as u16)
        } else {
            None
        }
    }

    fn name_table_rva(&self) -> Option<u32> {
        if self.import_by_name() {
            Some(self.0.to_le() as u32 & 0x7FFF_FFFF)
        } else {
            None
        }
    }
}

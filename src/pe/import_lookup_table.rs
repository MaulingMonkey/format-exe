use crate::*;
use crate::pe::RVA;



/// ∑ [ordinal](Self::ordinal) | [name_table_rva](Self::name_table_rva) | [is_eot](Self::is_eot) (impl by [ImportLookupTableEntry32], [ImportLookupTableEntry64])<br>
/// Describes the import of a single DLL symbol such as `"XInputGetState"`<br>
/// <br>
///
/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-lookup-table>
pub trait ImportLookupTableEntry {
    /// `true` if `self.0` == `0` (end of the table)
    fn is_eot           (&self) -> bool;

    /// <code>Some([RVA])</code> if the function is imported by name (typical)
    fn name_table_rva   (&self) -> Option<RVA>;

    /// <code>Some([u16])</code> if the function is imported by [ordinal/index rather than by name](https://learn.microsoft.com/en-us/cpp/build/exporting-functions-from-a-dll-by-ordinal-rather-than-by-name)
    fn ordinal          (&self) -> Option<u16>;
}

/// ∑ [ordinal](Self::ordinal) | [name_table_rva](Self::name_table_rva) | [is_eot](Self::is_eot) (impl [ImportLookupTableEntry])<br>
/// Describes the import of a single DLL symbol such as `"XInputGetState"` for 32-bit binaries<br>
/// <br>
///
/// ## References
/// *   <https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#import-lookup-table>
#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)] // TODO: PartialOrd, Ord, Hash
pub struct ImportLookupTableEntry32(u32le);

/// ∑ [ordinal](Self::ordinal) | [name_table_rva](Self::name_table_rva) | [is_eot](Self::is_eot) (impl [ImportLookupTableEntry])<br>
/// Describes the import of a single DLL symbol such as `"XInputGetState"` for 64-bit binaries<br>
/// <br>
///
/// As [`RVA`]s are limited to 32 bits, this could've reasonably been made a [u32], but it wasn't.
///
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

impl ImportLookupTableEntry32 {
    fn import_by_ordinal(&self) -> bool { (self.0.to_le() & (1 << 31)) != 0 }
    fn import_by_name   (&self) -> bool { !self.is_eot() && (self.0.to_le() & (1 << 31)) == 0 }
}

impl ImportLookupTableEntry for ImportLookupTableEntry32 {
    fn is_eot(&self) -> bool { self.0 == u32le::new(0) }

    fn ordinal(&self) -> Option<u16> {
        if self.import_by_ordinal() {
            Some(self.0.to_le() as u16)
        } else {
            None
        }
    }

    fn name_table_rva(&self) -> Option<RVA> {
        if self.import_by_name() {
            Some(RVA::new(self.0.to_le() as u32 & 0x7FFF_FFFF))
        } else {
            None
        }
    }
}

impl ImportLookupTableEntry64 {
    fn import_by_ordinal(&self) -> bool { (self.0.to_le() & (1 << 63)) != 0 }
    fn import_by_name   (&self) -> bool { !self.is_eot() && (self.0.to_le() & (1 << 31)) == 0 }
}

impl ImportLookupTableEntry for ImportLookupTableEntry64 {
    fn is_eot(&self) -> bool { self.0 == u64le::new(0) }

    fn ordinal(&self) -> Option<u16> {
        if self.import_by_ordinal() {
            Some(self.0.to_le() as u16)
        } else {
            None
        }
    }

    fn name_table_rva(&self) -> Option<RVA> {
        if self.import_by_name() {
            Some(RVA::new(self.0.to_le() as u32 & 0x7FFF_FFFF))
        } else {
            None
        }
    }
}

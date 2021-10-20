use crate::*;

use bytemuck::*;

use std::fmt::{self, Debug, Display, Formatter};



/// {
///     [major](struct.DataDirectory.html#structfield.major): C,
///     [minor](struct.DataDirectory.html#structfield.minor): C
/// } &nbsp;&nbsp;&nbsp;&nbsp; A version number in the style of e.g. "10.0".
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)] // TODO: PartialOrd/Ord/Hash
pub struct MajorMinorVersion<C> {
    pub major:  C,
    pub minor:  C,
}

impl<C> MajorMinorVersion<C> {
    pub fn into<O: From<C>>(self) -> MajorMinorVersion<O> {
        MajorMinorVersion {
            major: self.major.into(),
            minor: self.minor.into(),
        }
    }
}

impl<C: Display> Debug for MajorMinorVersion<C> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.major, self.minor)
    }
}

impl<C: Display> Display for MajorMinorVersion<C> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.major, self.minor)
    }
}

unsafe impl<C: Pod     > Pod      for MajorMinorVersion<C> {}
unsafe impl<C: Zeroable> Zeroable for MajorMinorVersion<C> {}

impl<C: FromMemory> FromMemory for MajorMinorVersion<C> {
    type Raw    = [<C as FromMemory>::Raw; 2];
    type Error  = <C as FromMemory>::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> {
        let [major, minor] = raw;
        let major = C::from_raw(major)?;
        let minor = C::from_raw(minor)?;
        Ok(Self { major, minor })
    }
}

use bytemuck::*;

use std::fmt::{self, *};



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

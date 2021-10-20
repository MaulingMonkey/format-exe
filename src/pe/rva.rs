use crate::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// **R**elative **V**irtual **A**ddress: A (32-bit) offset from the base
/// address of the loaded executable.
///
/// These are 32-bit values even in 64-bit executables - presumably you can't
/// scatter an executable's static data across more than 4GB of virtual memory.
/// Additionally, sometimes these source from bit packed fields - e.g. import
/// lookup tables use the lower 3**1** bits for RVAs - so some data might be
/// limited to a 2GB range or less.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RVA(u32);

impl RVA {
    pub const NULL : RVA = RVA(0);

    pub const fn new(rva: u32) -> Self { Self(rva) }

    pub fn to_u32   (self) -> u32   { self.0 as _ }
    pub fn to_u64   (self) -> u64   { self.0 as _ }
    pub fn to_usize (self) -> usize { self.0 as _ }
}

impl Debug for RVA {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "base+0x{:08x}", self.0)
    }
}

impl FromMemory for RVA {
    type Raw    = u32le;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(Self(raw.to_le())) }
}

// RVA +- offset

impl AddAssign<u32> for RVA { fn add_assign(&mut self, rhs: u32) { self.0 += rhs; } }
impl SubAssign<u32> for RVA { fn sub_assign(&mut self, rhs: u32) { self.0 -= rhs; } }
impl Add<u32> for RVA { type Output = RVA; fn add(self, rhs: u32) -> Self::Output { let mut rva = self; rva += rhs; rva } }
impl Add<RVA> for u32 { type Output = RVA; fn add(self, rhs: RVA) -> Self::Output { let mut rva = rhs; rva += self; rva } }
impl Sub<u32> for RVA { type Output = RVA; fn sub(self, rhs: u32) -> Self::Output { let mut rva = self; rva -= rhs; rva } }

// RVA - RVA

impl Sub<RVA> for RVA { type Output = u32; fn sub(self, rhs: RVA) -> Self::Output { self.to_u32() - rhs.to_u32() } }

// RVA <=> u*

impl From<RVA> for u32      { fn from(rva: RVA) -> Self { rva.0 as _ } }
impl From<RVA> for u64      { fn from(rva: RVA) -> Self { rva.0 as _ } }
impl From<RVA> for usize    { fn from(rva: RVA) -> Self { rva.0 as _ } }

impl From<u32> for RVA      { fn from(rva: u32) -> Self { Self(rva) } }

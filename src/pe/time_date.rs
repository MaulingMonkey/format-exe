use crate::*;

use bytemuck::*;

use std::fmt::{self, Debug, Formatter};
use std::time::{Duration, SystemTime};



/// Seconds elapsed since midnight, January 1, 1970, Universal Coordinated Time
#[repr(transparent)]
#[derive(Clone, Copy, Default)]
#[derive(Pod, Zeroable)]
pub struct TimeDate(u32le);

impl TimeDate {
    pub const UNIX_EPOCH : TimeDate = TimeDate(u32le::new(0));
}

impl From<TimeDate> for SystemTime {
    fn from(value: TimeDate) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_secs(value.0.to_le().into())
    }
}

impl Debug for TimeDate {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // TODO: better repr
        write!(fmt, "TimeDate({})", self.0)
    }
}

impl FromMemory for TimeDate {
    type Raw    = Self;
    type Error  = std::io::Error;
    fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> { Ok(raw) }
}

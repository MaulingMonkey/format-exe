use bytemuck::*;

use std::fmt::{self, *};



#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Reserved<const N: usize>([u8; N]);

unsafe impl<const N: usize> bytemuck::Pod       for Reserved<N> {}
unsafe impl<const N: usize> bytemuck::Zeroable  for Reserved<N> {}

impl<const N: usize> Default for Reserved<N> {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<const N: usize> Debug for Reserved<N> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[...]")
    }
}

pub(crate) use std::ops::Drop;

pub(crate) use zeroize::Zeroize;

pub(crate) type TABLE = [u32; 1024];

pub use buf::*;
mod buf;

pub use reg::*;
mod reg;

#[inline]
pub(crate) fn f1(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
pub(crate) fn f2(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

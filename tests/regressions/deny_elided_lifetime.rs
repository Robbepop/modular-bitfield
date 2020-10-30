#![deny(elided_lifetimes_in_paths)]

use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug)]
#[repr(u8)]
pub struct KeyFlags {
    pub certify: bool,
    pub fill: B7,
}

fn main() {}

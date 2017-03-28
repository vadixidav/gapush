#![feature(try_from)]

extern crate rand;
extern crate heapsize;

mod vec;
mod mem;

use vec::*;
use mem::TotalMemory;

use std::convert::TryFrom;

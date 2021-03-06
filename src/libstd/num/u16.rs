// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for unsigned 16-bits integers (`u16` type)

#![unstable]
#![doc(primitive = "u16")]

use from_str::FromStr;
use num::FromStrRadix;
use num::strconv;
use option::Option;

pub use core::u16::{BITS, BYTES, MIN, MAX};

uint_module!(u16)

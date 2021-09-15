// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

use crate::console;
use core::fmt;

//------------------------------------------------------------------------------
// Private Definitions
//------------------------------------------------------------------------------

/// A device for generating QEMU output.
struct QEMUOutput;

//------------------------------------------------------------------------------
// Private Code
//------------------------------------------------------------------------------

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which
/// in turn arn used to implement the `kernel`'s `print!` and `println!` maccros.
/// By implmenting `write_str()`, we get `write_fmt()` automatically.
///
/// See [`src/print.rs`].
///
/// [`src/print.rs`]: ../../print/index.html
impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

/// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}

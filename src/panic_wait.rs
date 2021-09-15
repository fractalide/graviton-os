// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

//! A panic handler that infinitely waits.

use crate::{cpu, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("\nKernel panic: {}", args);
    } else {
        println!("\nKernel panic!");
    }
    cpu::wait_forever()
}

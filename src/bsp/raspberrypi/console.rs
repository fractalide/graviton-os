// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

use crate::{console, bsp::device_driver};
use super::memory;
use core::fmt;

//------------------------------------------------------------------------------
// Public Code
//------------------------------------------------------------------------------

// In case of a panic, the panic handler uses this function to take a last stab at printing before the system is halted.
//
// We try to init panic-versions of the GPIO and the UART. The panic versions are not protected with synchronization primitives, which increases chances that we get to print something, even when the kernel's default GPIO or UART instances happen to be locked at the time of the panic.
//
// # Safety
//
// - Use only for printing during a panic.
pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut panic_gpio = device_driver::PanicGPIO::new(memory::map::mmio::GPIO_START);
    let mut panic_uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_START);
    panic_gpio.map_pl011_uart();
    panic_uart.init();
    panic_uart
}
pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}


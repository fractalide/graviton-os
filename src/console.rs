// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

//! System console.

//------------------------------------------------------------------------------
// Private Definitions
//------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    use core::fmt;
    /// Console write functions.
    pub trait Write {
        /// Write a Rust format string.
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }
    ///Console statistics.
    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }
    }
    /// Trait alias for a fully-fledged console.
    pub trait All = Write + Statistics;
}

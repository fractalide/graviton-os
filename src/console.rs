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
    /// Console write functions.
    ///
    /// `core::fmt::Write` is exactly what we need for now. Re-export it here because
    /// implementing `console::Write gives a better hint to the reader about the
    /// intention
    pub use core::fmt::Write;
}

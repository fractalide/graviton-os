// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

//! Boot code.

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/boot.rs"]
mod arch_boot;

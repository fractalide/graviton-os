// SPDX-License-Identifier: MIT OR APACHE-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Sublicensed from selected MIT to MPLv2

use core::{marker::PhantomData, ops};

//-------------------------------------------------------------------------------------
// Public Definitions
//-------------------------------------------------------------------------------------

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

//-------------------------------------------------------------------------------------
// Public Code
//-------------------------------------------------------------------------------------

impl<T> MMIODerefWrapper<T> {
    /// Create an instance
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom:PhantomData,
        }
    }
}
impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}

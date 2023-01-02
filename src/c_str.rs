// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! This mod is migrated from `std::ffi::CString` in order to support `no_std` feature.
//!
//! Note that `CStr` is moved from `std` intto `core` crate and
//! `CString` is moved into `alloc` in Rust 1.64.0.
//! This mod is kept to be compatible with rustc 1.46 and will be removed in middle of 2024.

use alloc::ffi::CString as StdCString;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::fmt;
use core::ptr;

pub struct CString(StdCString);

impl CString {
    #[inline]
    #[must_use]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.0.as_bytes_with_nul()
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[must_use]
    pub unsafe fn from_vec_unchecked(v: Vec<u8>) -> Self {
        unsafe { Self(StdCString::from_vec_unchecked(v)) }
    }

    #[must_use]
    pub unsafe fn from_vec_with_nul_unchecked(v: Vec<u8>) -> Self {
        unsafe { Self(StdCString::from_vec_with_nul_unchecked(v)) }
    }
}

pub type CStr = [u8];

impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl From<CString> for Vec<u8> {
    #[inline]
    fn from(s: CString) -> Self {
        s.0.into_bytes()
    }
}

#[repr(C)]
pub struct CStringArray {
    items: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CStringArray {
    pub fn new<S: AsRef<CStr>>(args: &[S]) -> Self {
        let items: Vec<CString> = args
            .iter()
            .map(|s| CString(StdCString::new(s.as_ref().to_vec()).unwrap()))
            .collect();
        let mut ptrs: Vec<*const c_char> = items.iter().map(|s| s.0.as_ptr()).collect();
        ptrs.push(ptr::null());

        Self { items, ptrs }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut result = CStringArray {
            items: Vec::with_capacity(capacity),
            ptrs: Vec::with_capacity(capacity + 1),
        };
        result.ptrs.push(ptr::null());
        result
    }

    pub fn push(&mut self, item: CString) {
        let l = self.ptrs.len();
        self.ptrs[l - 1] = item.0.as_ptr();
        self.ptrs.push(ptr::null());
        self.items.push(item);
    }

    pub fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_ptr()
    }

    pub fn as_bytes_ptr(&self) -> usize {
        println!(
            "item size: {}, ptr size: {}",
            self.items.len(),
            self.ptrs.len()
        );
        for _item in &self.items {
            println!("item: {:?}", item.0);
        }
        for _ptr in &self.ptrs {
            println!("ptr: {}", *ptr as usize);
        }
        self.as_ptr() as usize
    }
}

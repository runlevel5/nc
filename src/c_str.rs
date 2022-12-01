// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! This mod is migrated from `std::ffi::CString` in order to support `no_std` feature.
//!
//! Note that `CStr` is moved from `std` intto `core` crate and
//! `CString` is moved into `alloc` in Rust 1.64.0.
//! This mod is kept to be compatible with rustc 1.46 and will be removed in middle of 2024.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write;
use core::mem;
use core::ops;
use core::ptr;

pub struct CString {
    inner: Box<[u8]>,
}

pub struct CStr {
    inner: [u8],
}

impl CString {
    pub fn from_cstr(s: &CStr) -> Self {
        Self::new(s.inner.to_vec())
    }

    pub fn new<T: Into<Vec<u8>>>(t: T) -> Self {
        let mut v = t.into();
        v.reserve_exact(1);
        v.push(0);
        Self {
            inner: v.into_boxed_slice(),
        }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        let mut v: Vec<u8> = vec![0; cap];
        v.reserve_exact(1);
        v.push(0);
        Self {
            inner: v.into_boxed_slice(),
        }
    }

    #[must_use]
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.into_inner().into_vec()
    }

    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[..self.inner.len() - 1]
    }

    #[must_use]
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        &self.inner
    }

    #[must_use]
    #[inline]
    #[allow(clippy::borrow_deref_ref)]
    pub fn as_c_str(&self) -> &CStr {
        &*self
    }

    #[must_use]
    pub fn into_boxed_c_str(self) -> Box<CStr> {
        unsafe { Box::from_raw(Box::into_raw(self.into_inner()) as *mut CStr) }
    }

    #[must_use]
    fn into_inner(self) -> Box<[u8]> {
        let this = mem::ManuallyDrop::new(self);
        unsafe { ptr::read(&this.inner) }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_bytes_with_nul().len() - 1
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        // TODO(Shaohua): Check null bytes
        self.as_bytes_with_nul().len() == 0
    }

    #[must_use]
    pub fn strim_into_bytes(self) -> Vec<u8> {
        let mut vec = self.into_inner().into_vec();
        let mut nul_idx = 0;
        for v in &vec {
            if v == &0 {
                break;
            }
            nul_idx += 1;
        }
        vec.resize(nul_idx, 0);
        vec
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        let mut vec = self.into_inner().into_vec();
        let _nul = vec.pop();
        vec
    }

    #[must_use]
    pub unsafe fn from_vec_with_nul_unchecked(v: Vec<u8>) -> Self {
        unsafe { Self::_from_vec_with_nul_unchecked(v) }
    }

    unsafe fn _from_vec_with_nul_unchecked(v: Vec<u8>) -> Self {
        Self {
            inner: v.into_boxed_slice(),
        }
    }

    #[must_use]
    pub unsafe fn from_vec_unchecked(v: Vec<u8>) -> Self {
        unsafe { Self::_from_vec_unchecked(v) }
    }

    unsafe fn _from_vec_unchecked(mut v: Vec<u8>) -> Self {
        v.reserve_exact(1);
        v.push(0);
        Self {
            inner: v.into_boxed_slice(),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
}

impl CStr {
    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const [u8] as *const Self) }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes as *const [u8] as *const Self)
    }

    #[must_use]
    pub fn to_bytes(&self) -> &[u8] {
        let bytes = self.to_bytes_with_nul();
        &bytes[..bytes.len() - 1]
    }

    #[must_use]
    #[allow(clippy::borrow_as_ptr)]
    #[allow(clippy::missing_const_for_fn)]
    // NOTE(Shaohua): const unsafe feature is not available in rustc 1.46 stable.
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        unsafe { &*(&self.inner as *const [u8]) }
    }
}

impl AsRef<CStr> for str {
    #[inline]
    fn as_ref(&self) -> &CStr {
        CStr::new(self)
    }
}

impl AsRef<CStr> for String {
    #[inline]
    fn as_ref(&self) -> &CStr {
        CStr::new(self)
    }
}

impl AsRef<CStr> for [u8] {
    #[inline]
    fn as_ref(&self) -> &CStr {
        CStr::new(self)
    }
}

impl ops::Deref for CString {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
}

impl Drop for CString {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            *self.inner.get_unchecked_mut(0) = 0;
        }
    }
}

impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for CStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for byte in self.to_bytes().iter() {
            f.write_char(*byte as char)?;
        }
        write!(f, "\"")
    }
}

impl From<CString> for Vec<u8> {
    #[inline]
    fn from(s: CString) -> Self {
        s.into_bytes()
    }
}

#[repr(C)]
pub struct CStringList {
    args: Vec<CString>,
    argv: Vec<*const u8>,
}

impl CStringList {
    pub fn as_bytes_ptr(&self) -> usize {
        self.argv.as_ptr() as usize
    }

    pub fn push(&mut self, item: CString) {
        let l = self.argv.len();
        self.argv[l - 1] = item.as_ptr();
        self.argv.push(ptr::null());
        self.args.push(item);
    }

    pub fn as_ptr(&self) -> *const *const u8 {
        self.argv.as_ptr()
    }

    pub fn new<S: AsRef<CStr>>(args: &[S]) -> Self {
        let args: Vec<CString> = args
            .iter()
            .map(|s| CString::from_cstr(s.as_ref()))
            .collect();
        let mut argv: Vec<*const u8> = args.iter().map(|s| s.as_ptr()).collect();
        argv.push(ptr::null());

        Self { args, argv }
    }
}

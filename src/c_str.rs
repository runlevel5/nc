// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Re-export `CStr` and `CString`

#[cfg(feature = "std")]
pub use std::ffi::{CStr, CString};

#[cfg(not(feature = "std"))]
pub use alloc::ffi::CString;
#[cfg(not(feature = "std"))]
pub use core::ffi::CStr;

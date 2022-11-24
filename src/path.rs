// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use alloc::string::String;
use alloc::vec::Vec;

use crate::c_str::CString;

/// To fill the gap between path in rust and `*const char` in c.
///
/// A enum type is used to reduce memory copy.
pub enum PathBuf<'a> {
    WithoutNil(CString),
    WithNil(&'a [u8]),
}

impl<'a> PathBuf<'_> {
    pub fn as_bytes_ptr(&self) -> usize {
        match self {
            Self::WithoutNil(cstr) => cstr.as_bytes_with_nul().as_ptr() as usize,
            Self::WithNil(slice) => slice.as_ptr() as usize,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::WithoutNil(cstr) => cstr.as_bytes_with_nul().len(),
            Self::WithNil(slice) => slice.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::WithoutNil(cstr) => cstr.as_bytes_with_nul().is_empty(),
            Self::WithNil(slice) => slice.is_empty(),
        }
    }

    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::WithoutNil(cstr) => cstr.as_bytes_with_nul(),
            Self::WithNil(slice) => slice,
        }
    }
}

/// Reimplementation of `std::path::Path`.
pub struct Path {
    internal: [u8],
}

impl Path {
    pub unsafe fn to_own(&self) -> PathBuf {
        if self.internal.is_empty() || self.internal[self.len() - 1] != 0 {
            PathBuf::WithoutNil(CString::from_vec_unchecked(self.internal.to_vec()))
        } else {
            PathBuf::WithNil(&self.internal)
        }
    }
}

impl Path {
    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const [u8] as *const Self) }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.internal.len()
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}

impl AsRef<Path> for str {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for String {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl From<&Path> for Vec<u8> {
    fn from(path: &Path) -> Self {
        path.internal.to_vec()
    }
}

#[cfg(feature = "std")]
mod with_std {
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::os::unix::ffi::OsStrExt;
    use std::path;

    use super::Path;

    impl AsRef<Path> for OsStr {
        #[inline]
        fn as_ref(&self) -> &Path {
            Path::new(self.as_bytes())
        }
    }
    impl AsRef<Path> for Cow<'_, OsStr> {
        #[inline]
        fn as_ref(&self) -> &Path {
            Path::new(self.as_bytes())
        }
    }
    impl AsRef<Path> for OsString {
        #[inline]
        fn as_ref(&self) -> &Path {
            Path::new(self.as_bytes())
        }
    }
    impl AsRef<Path> for path::PathBuf {
        fn as_ref(&self) -> &Path {
            self.as_path().as_ref()
        }
    }
    impl AsRef<Path> for path::Path {
        #[inline]
        fn as_ref(&self) -> &Path {
            Path::new(self.as_os_str().as_bytes())
        }
    }
}

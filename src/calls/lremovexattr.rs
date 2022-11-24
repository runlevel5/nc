/// Remove an extended attribute.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-lremovexattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::lremovexattr(path, attr_name) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// ```
pub unsafe fn lremovexattr<P: AsRef<Path>>(filename: P, name: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall2(SYS_LREMOVEXATTR, filename_ptr, name_ptr).map(drop)
}

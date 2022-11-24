/// Set extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fsetxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::fsetxattr(
///         fd,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fsetxattr<P: AsRef<Path>>(
    fd: i32,
    name: P,
    value: usize,
    size: size_t,
    flags: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(SYS_FSETXATTR, fd, name_ptr, value, size, flags).map(drop)
}

/// Get extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fgetxattr";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
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
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::fgetxattr(fd, attr_name, buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(attr_value.len() as nc::ssize_t));
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(attr_value.as_bytes(), &buf[..attr_len]);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fgetxattr<P: AsRef<Path>>(
    fd: i32,
    name: P,
    value: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall4(SYS_FGETXATTR, fd, name_ptr, value, size).map(|ret| ret as ssize_t)
}

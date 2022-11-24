/// Create a file.
///
/// equals to call `open()` with flags `O_CREAT|O_WRONLY|O_TRUNC`.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-creat-file";
/// let fd = unsafe { nc::creat(path, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn creat<P: AsRef<Path>>(filename: P, mode: mode_t) -> Result<i32, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall2(SYS_CREAT, filename_ptr, mode).map(|ret| ret as i32)
}

/// Change name or location of a file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-renameat";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_path = "/tmp/nc-renameat-new";
/// let ret = unsafe { nc::renameat(nc::AT_FDCWD, path, nc::AT_FDCWD, new_path) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn renameat<P: AsRef<Path>>(
    olddfd: i32,
    oldfilename: P,
    newdfd: i32,
    newfilename: P,
) -> Result<(), Errno> {
    let olddfd = olddfd as usize;
    let oldfilename = PathBuf::new(oldfilename);
    let oldfilename_ptr = oldfilename.as_ptr() as usize;
    let newdfd = newdfd as usize;
    let newfilename = PathBuf::new(newfilename);
    let newfilename_ptr = newfilename.as_ptr() as usize;
    syscall4(
        SYS_RENAMEAT,
        olddfd,
        oldfilename_ptr,
        newdfd,
        newfilename_ptr,
    )
    .map(drop)
}

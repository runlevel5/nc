/// Change the root directory.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::chroot("/") };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn chroot<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_CHROOT, filename_ptr).map(drop)
}

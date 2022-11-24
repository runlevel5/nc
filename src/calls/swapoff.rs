/// Stop swapping to file/device.
///
/// # Example
///
/// ```
/// let filename = "/dev/sda-no-exist";
/// let ret = unsafe { nc::swapoff(filename) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn swapoff<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_SWAPOFF, filename_ptr).map(drop)
}

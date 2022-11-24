/// Get file status.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat64_t::default();
/// let ret = unsafe { nc::fstatat64(nc::AT_FDCWD, path, &mut stat, nc::AT_SYMLINK_NOFOLLOW) };
/// assert!(ret.is_ok());
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn fstatat64<P: AsRef<Path>>(
    dfd: i32,
    filename: P,
    statbuf: &mut stat64_t,
    flag: i32,
) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat64_t as usize;
    let flag = flag as usize;
    syscall4(SYS_FSTATAT64, dfd, filename_ptr, statbuf_ptr, flag).map(drop)
}

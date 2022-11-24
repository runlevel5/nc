/// Get file status about a file.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat_t::default();
/// let ret = unsafe { nc::stat(path, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn stat<P: AsRef<Path>>(filename: P, statbuf: &mut stat_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat_t as usize;
    syscall2(SYS_STAT, filename_ptr, statbuf_ptr).map(drop)
}

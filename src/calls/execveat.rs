/// Execute a new program relative to a directory file descriptor.
///
/// # Example
///
/// ```
/// let args = ["-l", "/"];
/// let env = ["LANG=en_US.UTF-8"];
/// let ret = unsafe { nc::execveat(nc::AT_FDCWD, "/bin/ls", &args, &env, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn execveat<P: AsRef<Path>, S: AsRef<CStr>>(
    fd: i32,
    filename: P,
    argv: &[S],
    env: &[S],
    flags: i32,
) -> Result<(), Errno> {
    // And return value might be changed too.

    let fd = fd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let argv_ptr = CStringList::new(argv).as_bytes_ptr();
    let env_ptr = CStringList::new(env).as_bytes_ptr();
    let flags = flags as usize;
    syscall5(SYS_EXECVEAT, fd, filename_ptr, argv_ptr, env_ptr, flags).map(drop)
}

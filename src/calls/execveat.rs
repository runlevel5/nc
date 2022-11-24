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
    let argv_ptr = to_c_str_vec(argv).as_ptr() as usize;
    let env_ptr = to_c_str_vec(env).as_ptr() as usize;
    let flags = flags as usize;
    syscall5(SYS_EXECVEAT, fd, filename_ptr, argv_ptr, env_ptr, flags).map(drop)
}

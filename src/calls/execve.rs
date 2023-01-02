/// Execute a new program.
///
/// And return value might be changed too.
///
/// # Example
///
/// ```
/// let args = ["-l", "/"];
/// let env = ["LANG=en_US.UTF-8"];
/// let ret = unsafe { nc::execve("/bin/ls", &args, &env) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn execve<P: AsRef<Path>, S: AsRef<CStr>>(
    filename: P,
    argv: &[S],
    env: &[S],
) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let argv_ptr = CStringArray::new(argv).as_bytes_ptr();
    let env_ptr = CStringArray::new(env).as_bytes_ptr();
    syscall3(SYS_EXECVE, filename_ptr, argv_ptr, env_ptr).map(drop)
}

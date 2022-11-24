/// Pick a superblock into a context for reconfiguration.
pub unsafe fn fspick<P: AsRef<Path>>(dfd: i32, path: P, flags: i32) -> Result<i32, Errno> {
    let dfd = dfd as usize;
    let path = PathBuf::new(path);
    let path_ptr = path.as_ptr() as usize;
    let flags = flags as usize;
    syscall3(SYS_FSPICK, dfd, path_ptr, flags).map(|ret| ret as i32)
}

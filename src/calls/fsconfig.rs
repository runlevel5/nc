/// Set parameters and trigger actions on a context.
pub unsafe fn fsconfig<P: AsRef<Path>>(
    fd: i32,
    cmd: u32,
    key: P,
    value: P,
    aux: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let cmd = cmd as usize;
    let key = PathBuf::new(key);
    let key_ptr = key.as_ptr() as usize;
    let value = PathBuf::new(value);
    let value_ptr = value.as_ptr() as usize;
    let aux = aux as usize;
    syscall5(SYS_FSCONFIG, fd, cmd, key_ptr, value_ptr, aux).map(drop)
}

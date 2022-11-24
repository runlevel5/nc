/// Create an anonymous file.
pub unsafe fn memfd_create<P: AsRef<Path>>(name: P, flags: u32) -> Result<i32, Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_MEMFD_CREATE, name_ptr, flags).map(|ret| ret as i32)
}

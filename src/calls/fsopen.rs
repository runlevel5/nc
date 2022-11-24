/// Open a filesystem by name so that it can be configured for mounting.
pub unsafe fn fsopen<P: AsRef<Path>>(fs_name: P, flags: u32) -> Result<(), Errno> {
    let fs_name = PathBuf::new(fs_name);
    let fs_name_ptr = fs_name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_FSOPEN, fs_name_ptr, flags).map(drop)
}

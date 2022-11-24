/// Mount filesystem.
///
/// # Example
///
/// ```
/// let target_dir = "/tmp/nc-mount";
/// let ret = unsafe { nc::mkdirat(nc::AT_FDCWD, target_dir, 0o755) };
/// assert!(ret.is_ok());
///
/// let src_dir = "/etc";
/// let fs_type = "";
/// let mount_flags = nc::MS_BIND | nc::MS_RDONLY;
/// let data = 0;
/// let ret = unsafe { nc::mount(src_dir, target_dir, fs_type, mount_flags, data) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
///
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, target_dir, nc::AT_REMOVEDIR) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mount<P: AsRef<Path>>(
    dev_name: P,
    dir_name: P,
    fs_type: P,
    flags: usize,
    data: usize,
) -> Result<(), Errno> {
    let dev_name = PathBuf::new(dev_name);
    let dev_name_ptr = dev_name.as_ptr() as usize;
    let dir_name = PathBuf::new(dir_name);
    let dir_name_ptr = dir_name.as_ptr() as usize;
    let fs_type = PathBuf::new(fs_type);
    let fs_type_ptr = fs_type.as_ptr() as usize;
    syscall5(
        SYS_MOUNT,
        dev_name_ptr,
        dir_name_ptr,
        fs_type_ptr,
        flags,
        data,
    )
    .map(drop)
}

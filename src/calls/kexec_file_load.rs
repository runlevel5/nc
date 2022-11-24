/// Load a new kernel for later execution.
pub unsafe fn kexec_file_load<P: AsRef<Path>>(
    kernel_fd: i32,
    initrd_fd: i32,
    cmdline: P,
    flags: usize,
) -> Result<(), Errno> {
    let kernel_fd = kernel_fd as usize;
    let initrd_fd = initrd_fd as usize;
    let cmdline_len = cmdline.as_ref().len();
    let cmdline = PathBuf::new(cmdline);
    let cmdline_ptr = cmdline.as_ptr() as usize;
    syscall5(
        SYS_KEXEC_FILE_LOAD,
        kernel_fd,
        initrd_fd,
        cmdline_len,
        cmdline_ptr,
        flags,
    )
    .map(drop)
}

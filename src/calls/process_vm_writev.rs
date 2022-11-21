/// Transfer data between process address spaces
pub unsafe fn process_vm_writev(
    pid: pid_t,
    lvec: &[iovec_t],
    rvec: &[iovec_t],
    flags: i32,
) -> Result<ssize_t, Errno> {
    let pid = pid as usize;
    let lvec_ptr = lvec.as_ptr() as usize;
    let lvec_len = lvec.len();
    let rvec_ptr = rvec.as_ptr() as usize;
    let rvec_len = rvec.len();
    let flags = flags as usize;
    syscall6(
        SYS_PROCESS_VM_WRITEV,
        pid,
        lvec_ptr,
        lvec_len,
        rvec_ptr,
        rvec_len,
        flags,
    )
    .map(|ret| ret as ssize_t)
}

// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::similar_names)]
#![allow(clippy::wildcard_imports)]

extern crate alloc;

use crate::c_str::{to_c_str_vec, CStr};
use crate::path::{Path, PathBuf};
use crate::syscalls::*;
use crate::sysno::*;
use crate::types::*;

/// Accept a connection on a socket.
pub unsafe fn accept4(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
    flags: i32,
) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let addr_ptr = addr as *mut sockaddr_in_t as usize;
    let addrlen_ptr = addrlen as *mut socklen_t as usize;
    let flags = flags as usize;
    syscall4(SYS_ACCEPT4, sockfd, addr_ptr, addrlen_ptr, flags).map(drop)
}

/// Check user's permission for a file.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::access("/etc/passwd", nc::F_OK) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::access("/etc/passwd", nc::X_OK) };
/// assert!(ret.is_err());
/// ```
pub unsafe fn access<P: AsRef<Path>>(filename: P, mode: i32) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall2(SYS_ACCESS, filename_ptr, mode).map(drop)
}

/// Switch process accounting.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-acct";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::acct(path) };
/// assert_eq!(ret, Err(nc::EPERM));
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn acct<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_ACCT, filename_ptr).map(drop)
}

/// Add a key to the kernel's key management facility.
pub unsafe fn add_key<P: AsRef<Path>>(
    type_: P,
    description: P,
    payload: usize,
    plen: size_t,
    dest_keyring: key_serial_t,
) -> Result<key_serial_t, Errno> {
    let type_ = PathBuf::new(type_);
    let type_ptr = type_.as_ptr() as usize;
    let description = PathBuf::new(description);
    let description_ptr = description.as_ptr() as usize;
    let dest_keyring = dest_keyring as usize;
    syscall5(
        SYS_ADD_KEY,
        type_ptr,
        description_ptr,
        payload,
        plen,
        dest_keyring,
    )
    .map(|ret| ret as key_serial_t)
}

/// Tune kernel clock. Returns clock state on success.
///
/// # Example
///
/// ```
/// let mut tm = nc::timex_t::default();
/// let ret = unsafe { nc::adjtimex(&mut tm) };
/// assert!(ret.is_ok());
/// assert!(tm.time.tv_sec > 1611552896);
/// ```
pub unsafe fn adjtimex(buf: &mut timex_t) -> Result<i32, Errno> {
    let buf_ptr = buf as *mut timex_t as usize;
    syscall1(SYS_ADJTIMEX, buf_ptr).map(|ret| ret as i32)
}

/// Set an alarm clock for delivery of a signal.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
/// }
///
/// let sa = nc::sigaction_t {
///     sa_handler: handle_alarm as nc::sighandler_t,
///     ..nc::sigaction_t::default()
/// };
/// let mut old_sa = nc::sigaction_t::default();
/// let ret = unsafe { nc::rt_sigaction(nc::SIGALRM, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
/// assert!(ret.is_ok());
/// let remaining = unsafe { nc::alarm(1) };
/// let mask = nc::sigset_t::default();
/// let ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
/// assert_eq!(ret, Err(nc::EINTR));
/// assert_eq!(remaining, 0);
/// ```
#[must_use]
pub unsafe fn alarm(seconds: u32) -> u32 {
    let seconds = seconds as usize;
    // This function is always successful.
    syscall1(SYS_ALARM, seconds).expect("alarm() failed") as u32
}

/// Set architecture-specific thread state.
pub unsafe fn arch_prctl(code: i32, arg2: usize) -> Result<(), Errno> {
    let code = code as usize;
    syscall2(SYS_ARCH_PRCTL, code, arg2).map(drop)
}

/// Start, flush or tune buffer-dirty-flush daemon.
/// There are no bdflush tunables left.  But distributions are
/// still running obsolete flush daemons, so we terminate them here.
///
/// Use of `bdflush()` is deprecated and will be removed in a future kernel.
/// The `flush-X` kernel threads fully replace bdflush daemons and this call.
/// Deprecated.
pub unsafe fn bdflush() {
    core::unimplemented!();
    // syscall0(SYS_BDFLUSH);
}

/// Bind a name to a socket.
pub unsafe fn bind(sockfd: i32, addr: &sockaddr_in_t, addrlen: socklen_t) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let addr_ptr = addr as *const sockaddr_in_t as usize;
    let addrlen = addrlen as usize;
    syscall3(SYS_BIND, sockfd, addr_ptr, addrlen).map(drop)
}

/// Perform a command on an extended BPF map or program
pub unsafe fn bpf(cmd: i32, attr: &mut bpf_attr_t, size: u32) -> Result<i32, Errno> {
    let cmd = cmd as usize;
    let attr_ptr = attr as *mut bpf_attr_t as usize;
    let size = size as usize;
    syscall3(SYS_BPF, cmd, attr_ptr, size).map(|ret| ret as i32)
}

/// Change data segment size.
pub unsafe fn brk(addr: usize) -> Result<(), Errno> {
    syscall1(SYS_BRK, addr).map(drop)
}

/// Get capabilities of thread.
pub unsafe fn capget(
    hdrp: &mut cap_user_header_t,
    data: &mut cap_user_data_t,
) -> Result<(), Errno> {
    let hdrp_ptr = hdrp as *mut cap_user_header_t as usize;
    let data_ptr = data as *mut cap_user_data_t as usize;
    syscall2(SYS_CAPGET, hdrp_ptr, data_ptr).map(drop)
}

/// Set capabilities of thread.
pub unsafe fn capset(hdrp: &mut cap_user_header_t, data: &cap_user_data_t) -> Result<(), Errno> {
    let hdrp_ptr = hdrp as *mut cap_user_header_t as usize;
    let data_ptr = data as *const cap_user_data_t as usize;
    syscall2(SYS_CAPSET, hdrp_ptr, data_ptr).map(drop)
}

/// Change working directory.
///
/// # Example
///
/// ```
/// let path = "/tmp";
/// // Open folder directly.
/// let ret = unsafe { nc::chdir(path) };
/// assert!(ret.is_ok());
///
/// let mut buf = [0_u8; nc::PATH_MAX as usize + 1];
/// let ret = unsafe { nc::getcwd(buf.as_mut_ptr() as usize, buf.len()) };
/// assert!(ret.is_ok());
/// // Remove null-terminal char.
/// let path_len = ret.unwrap() as usize - 1;
/// let new_cwd = std::str::from_utf8(&buf[..path_len]);
/// assert_eq!(new_cwd, Ok(path));
/// ```
pub unsafe fn chdir<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_CHDIR, filename_ptr).map(drop)
}

/// Change permissions of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-chmod";
/// let fd = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644,
///     )
/// };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::chmod(filename, 0o600) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn chmod<P: AsRef<Path>>(filename: P, mode: mode_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall2(SYS_CHMOD, filename_ptr, mode).map(drop)
}

/// Change ownership of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-chown";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, filename, nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::chown(filename, 0, 0) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn chown<P: AsRef<Path>>(filename: P, user: uid_t, group: gid_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let user = user as usize;
    let group = group as usize;
    syscall3(SYS_CHOWN, filename_ptr, user, group).map(drop)
}

/// Change the root directory.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::chroot("/") };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn chroot<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_CHROOT, filename_ptr).map(drop)
}

/// Tune kernel clock. Returns clock state on success.
///
/// # Example
///
/// ```
/// let mut tm = nc::timex_t::default();
/// let ret = unsafe { nc::clock_adjtime(nc::CLOCK_REALTIME, &mut tm) };
/// assert!(ret.is_ok());
/// assert!(tm.time.tv_sec > 1611552896);
/// ```
pub unsafe fn clock_adjtime(which_clock: clockid_t, tx: &mut timex_t) -> Result<(), Errno> {
    let which_clock = which_clock as usize;
    let tx_ptr = tx as *mut timex_t as usize;
    syscall2(SYS_CLOCK_ADJTIME, which_clock, tx_ptr).map(drop)
}

/// Get resolution(precision) of the specific clock.
///
/// # Example
///
/// ```
/// let mut tp = nc::timespec_t::default();
/// let ret = unsafe { nc::clock_getres(nc::CLOCK_BOOTTIME, &mut tp) };
/// assert!(ret.is_ok());
/// assert!(tp.tv_nsec > 0);
/// ```
pub unsafe fn clock_getres(which_clock: clockid_t, tp: &mut timespec_t) -> Result<(), Errno> {
    let which_clock = which_clock as usize;
    let tp_ptr = tp as *mut timespec_t as usize;
    syscall2(SYS_CLOCK_GETRES, which_clock, tp_ptr).map(drop)
}

/// Get time of specific clock.
///
/// # Example
///
/// ```
/// let mut tp = nc::timespec_t::default();
/// let ret = unsafe { nc::clock_gettime(nc::CLOCK_REALTIME_COARSE, &mut tp) };
/// assert!(ret.is_ok());
/// assert!(tp.tv_sec > 0);
/// ```
pub unsafe fn clock_gettime(which_clock: clockid_t, tp: &mut timespec_t) -> Result<(), Errno> {
    let which_clock = which_clock as usize;
    let tp_ptr = tp as *mut timespec_t as usize;
    syscall2(SYS_CLOCK_GETTIME, which_clock, tp_ptr).map(drop)
}

/// High resolution sleep with a specific clock.
///
/// # Example
///
/// ```
/// let t = nc::timespec_t {
///     tv_sec: 1,
///     tv_nsec: 0,
/// };
/// let mut rem = nc::timespec_t::default();
/// let ret = unsafe { nc::clock_nanosleep(nc::CLOCK_MONOTONIC, 0, &t, &mut rem) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn clock_nanosleep(
    which_clock: clockid_t,
    flags: i32,
    rqtp: &timespec_t,
    rmtp: &mut timespec_t,
) -> Result<(), Errno> {
    let which_clock = which_clock as usize;
    let flags = flags as usize;
    let rqtp_ptr = rqtp as *const timespec_t as usize;
    let rmtp_ptr = rmtp as *mut timespec_t as usize;
    syscall4(SYS_CLOCK_NANOSLEEP, which_clock, flags, rqtp_ptr, rmtp_ptr).map(drop)
}

/// Set time of specific clock.
///
/// # Example
///
/// ```
/// let mut tp = nc::timespec_t::default();
/// let ret = unsafe { nc::clock_gettime(nc::CLOCK_REALTIME, &mut tp) };
/// assert!(ret.is_ok());
/// assert!(tp.tv_sec > 0);
/// let ret = unsafe { nc::clock_settime(nc::CLOCK_REALTIME, &tp) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn clock_settime(which_clock: clockid_t, tp: &timespec_t) -> Result<(), Errno> {
    let which_clock = which_clock as usize;
    let tp_ptr = tp as *const timespec_t as usize;
    syscall2(SYS_CLOCK_SETTIME, which_clock, tp_ptr).map(drop)
}

/// Create a child process.
pub unsafe fn clone(
    clone_flags: i32,
    newsp: usize,
    parent_tid: &mut i32,
    child_tid: &mut i32,
    tls: usize,
) -> Result<pid_t, Errno> {
    let clone_flags = clone_flags as usize;
    let parent_tid_ptr = parent_tid as *mut i32 as usize;
    let child_tid_ptr = child_tid as *mut i32 as usize;
    syscall5(
        SYS_CLONE,
        clone_flags,
        newsp,
        parent_tid_ptr,
        child_tid_ptr,
        tls,
    )
    .map(|ret| ret as pid_t)
}

/// New api to create child process.
///
/// # Example
///
/// ```
/// let mut args = nc::clone_args_t::default();
/// let mut pid_fd: i32 = -1;
/// args.exit_signal = nc::SIGCHLD as u64;
/// args.pidfd = &mut pid_fd as *mut i32 as usize as u64;
/// args.flags = nc::CLONE_PIDFD as u64 | nc::CLONE_PARENT_SETTID as u64;
/// let pid = unsafe { nc::clone3(&mut args, core::mem::size_of::<nc::clone_args_t>()) };
/// assert!(pid.is_ok());
/// ```
pub unsafe fn clone3(cl_args: &mut clone_args_t, size: size_t) -> Result<pid_t, Errno> {
    let cl_args_ptr = cl_args as *mut clone_args_t as usize;
    syscall2(SYS_CLONE3, cl_args_ptr, size).map(|ret| ret as pid_t)
}

/// Close a file descriptor.
///
/// # Example
///
/// ```
/// const STDERR_FD: i32 = 2;
/// let ret = unsafe { nc::close(STDERR_FD) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn close(fd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    syscall1(SYS_CLOSE, fd).map(drop)
}

/// Close all file descriptors in a given range
///
/// # Example
///
/// ```
/// const STDOUT_FD: u32 = 1;
/// const STDERR_FD: u32 = 2;
/// let ret = unsafe { nc::close_range(STDOUT_FD, STDERR_FD, nc::CLOSE_RANGE_CLOEXEC) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn close_range(first_fd: u32, last_fd: u32, flags: u32) -> Result<(), Errno> {
    let first = first_fd as usize;
    let last = last_fd as usize;
    let flags = flags as usize;
    syscall3(SYS_CLOSE_RANGE, first, last, flags).map(drop)
}

/// Initialize a connection on a socket.
pub unsafe fn connect(sockfd: i32, addr: &sockaddr_in_t, addrlen: socklen_t) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    // TODO(Shaohua): Use sockaddr_t generic type.
    let addr_ptr = addr as *const sockaddr_in_t as usize;
    let addrlen = addrlen as usize;
    syscall3(SYS_CONNECT, sockfd, addr_ptr, addrlen).map(drop)
}

/// Copy a range of data from one file to another.
///
/// # Example
///
/// ```
/// let path_in = "/etc/passwd";
/// let fd_in = unsafe { nc::openat(nc::AT_FDCWD, path_in, nc::O_RDONLY, 0) };
/// assert!(fd_in.is_ok());
/// let fd_in = fd_in.unwrap();
/// let path_out = "/tmp/nc-copy-file-range";
/// let fd_out = unsafe { nc::openat(nc::AT_FDCWD, path_out, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd_out.is_ok());
/// let fd_out = fd_out.unwrap();
/// let mut off_in = 0;
/// let mut off_out = 0;
/// let copy_len = 64;
/// let ret = unsafe { nc::copy_file_range(fd_in, &mut off_in, fd_out, &mut off_out, copy_len, 0) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(copy_len as nc::ssize_t));
/// let ret = unsafe { nc::close(fd_in) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd_out) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path_out, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn copy_file_range(
    fd_in: i32,
    off_in: &mut loff_t,
    fd_out: i32,
    off_out: &mut loff_t,
    len: size_t,
    flags: u32,
) -> Result<ssize_t, Errno> {
    let fd_in = fd_in as usize;
    let off_in_ptr = off_in as *mut loff_t as usize;
    let fd_out = fd_out as usize;
    let off_out_ptr = off_out as *mut loff_t as usize;
    let flags = flags as usize;
    syscall6(
        SYS_COPY_FILE_RANGE,
        fd_in,
        off_in_ptr,
        fd_out,
        off_out_ptr,
        len,
        flags,
    )
    .map(|ret| ret as ssize_t)
}

/// Create a file.
///
/// equals to call `open()` with flags `O_CREAT|O_WRONLY|O_TRUNC`.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-creat-file";
/// let fd = unsafe { nc::creat(path, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn creat<P: AsRef<Path>>(filename: P, mode: mode_t) -> Result<i32, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall2(SYS_CREAT, filename_ptr, mode).map(|ret| ret as i32)
}

/// Unlock a kernel module.
pub unsafe fn delete_module<P: AsRef<Path>>(name: P, flags: i32) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_DELETE_MODULE, name_ptr, flags).map(drop)
}

/// Create a copy of the file descriptor `oldfd`, using the lowest available
/// file descriptor.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-dup-file";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let fd_dup = unsafe { nc::dup(fd) };
/// assert!(fd_dup.is_ok());
/// let fd_dup = fd_dup.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd_dup) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn dup(oldfd: i32) -> Result<i32, Errno> {
    let oldfd = oldfd as usize;
    syscall1(SYS_DUP, oldfd).map(|ret| ret as i32)
}

/// Create a copy of the file descriptor `oldfd`, using the speficified file
/// descriptor `newfd`.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-dup2-file";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let newfd = 8;
/// let ret = unsafe { nc::dup2(fd, newfd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(newfd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn dup2(oldfd: i32, newfd: i32) -> Result<(), Errno> {
    let oldfd = oldfd as usize;
    let newfd = newfd as usize;
    syscall2(SYS_DUP2, oldfd, newfd).map(drop)
}

/// Save as `dup2()`, but can set the close-on-exec flag on `newfd`.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-dup3-file";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let newfd = 8;
/// let ret = unsafe { nc::dup3(fd, newfd, nc::O_CLOEXEC) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(newfd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn dup3(oldfd: i32, newfd: i32, flags: i32) -> Result<(), Errno> {
    let oldfd = oldfd as usize;
    let newfd = newfd as usize;
    let flags = flags as usize;
    syscall3(SYS_DUP3, oldfd, newfd, flags).map(drop)
}

/// Open an epoll file descriptor.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::epoll_create(32) };
/// assert!(ret.is_ok());
/// let poll_fd = ret.unwrap();
/// let ret = unsafe { nc::close(poll_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn epoll_create(size: i32) -> Result<i32, Errno> {
    let size = size as usize;
    syscall1(SYS_EPOLL_CREATE, size).map(|ret| ret as i32)
}

/// Open an epoll file descriptor.
///
/// # Example
///
/// ```
/// let poll_fd = unsafe { nc::epoll_create1(nc::EPOLL_CLOEXEC) };
/// assert!(poll_fd.is_ok());
/// let poll_fd = poll_fd.unwrap();
/// let ret = unsafe { nc::close(poll_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn epoll_create1(flags: i32) -> Result<i32, Errno> {
    let flags = flags as usize;
    syscall1(SYS_EPOLL_CREATE1, flags).map(|ret| ret as i32)
}

/// Control interface for an epoll file descriptor.
///
/// # Example
///
/// ```
/// let epfd = unsafe { nc::epoll_create1(nc::EPOLL_CLOEXEC) };
/// assert!(epfd.is_ok());
/// let epfd = epfd.unwrap();
/// let mut fds: [i32; 2] = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds, 0) };
/// assert!(ret.is_ok());
/// let mut event = nc::epoll_event_t::default();
/// event.events = nc::EPOLLIN | nc::EPOLLET;
/// event.data.fd = fds[0];
/// let ctl_ret = unsafe { nc::epoll_ctl(epfd, nc::EPOLL_CTL_ADD, fds[0], &mut event) };
/// assert!(ctl_ret.is_ok());
/// let ret = unsafe { nc::close(fds[0]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[1]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(epfd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn epoll_ctl(
    epfd: i32,
    op: i32,
    fd: i32,
    event: &mut epoll_event_t,
) -> Result<(), Errno> {
    let epfd = epfd as usize;
    let op = op as usize;
    let fd = fd as usize;
    let event_ptr = event as *mut epoll_event_t as usize;
    syscall4(SYS_EPOLL_CTL, epfd, op, fd, event_ptr).map(drop)
}

/// Wait for an I/O event on an epoll file descriptor.
///
/// # Example
///
/// ```
/// let epfd = unsafe { nc::epoll_create1(nc::EPOLL_CLOEXEC) };
/// assert!(epfd.is_ok());
/// let epfd = epfd.unwrap();
/// let mut fds: [i32; 2] = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds, 0) };
/// assert!(ret.is_ok());
/// let mut event = nc::epoll_event_t::default();
/// event.events = nc::EPOLLIN | nc::EPOLLET;
/// event.data.fd = fds[0];
/// let ctl_ret = unsafe { nc::epoll_ctl(epfd, nc::EPOLL_CTL_ADD, fds[0], &mut event) };
/// assert!(ctl_ret.is_ok());
///
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fds[1], msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
///
/// let mut events = vec![nc::epoll_event_t::default(); 4];
/// let events_len = events.len();
/// let timeout = 0;
/// let sigmask = nc::sigset_t::default();
/// let sigmask_size = core::mem::size_of_val(&sigmask);
/// let ret = unsafe {
///     nc::epoll_pwait(
///         epfd,
///         &mut events,
///         events_len as i32,
///         timeout,
///         &sigmask,
///         sigmask_size,
///     )
/// };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(1));
///
/// for event in &events {
///     // Ready to read
///     if event.events == nc::EPOLLIN {
///         let ready_fd = unsafe { event.data.fd };
///         assert_eq!(ready_fd, fds[0]);
///         let mut buf = vec![0_u8; 64];
///         let buf_len = buf.len();
///         let ret = unsafe { nc::read(ready_fd, buf.as_mut_ptr() as usize, buf_len) };
///         assert!(ret.is_ok());
///         let n_read = ret.unwrap() as usize;
///         assert_eq!(msg.as_bytes(), &buf[..n_read]);
///     }
/// }
///
/// let ret = unsafe { nc::close(fds[0]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[1]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(epfd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn epoll_pwait(
    epfd: i32,
    events: &mut [epoll_event_t],
    max_events: i32,
    timeout: i32,
    sigmask: &sigset_t,
    sigset_size: usize,
) -> Result<i32, Errno> {
    let epfd = epfd as usize;
    let events_ptr = events.as_mut_ptr() as usize;
    let max_events = max_events as usize;
    let timeout = timeout as usize;
    let sigmask_ptr = sigmask as *const sigset_t as usize;
    syscall6(
        SYS_EPOLL_PWAIT,
        epfd,
        events_ptr,
        max_events,
        timeout,
        sigmask_ptr,
        sigset_size,
    )
    .map(|ret| ret as i32)
}

/// Wait for an I/O event on an epoll file descriptor.
///
/// # Example
///
/// ```
/// let epfd = unsafe { nc::epoll_create1(nc::EPOLL_CLOEXEC) };
/// assert!(epfd.is_ok());
/// let epfd = epfd.unwrap();
/// let mut fds: [i32; 2] = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds, 0) };
/// assert!(ret.is_ok());
/// let mut event = nc::epoll_event_t::default();
/// event.events = nc::EPOLLIN | nc::EPOLLET;
/// event.data.fd = fds[0];
/// let ctl_ret = unsafe { nc::epoll_ctl(epfd, nc::EPOLL_CTL_ADD, fds[0], &mut event) };
/// assert!(ctl_ret.is_ok());
///
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fds[1], msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
///
/// let mut events = vec![nc::epoll_event_t::default(); 4];
/// let events_len = events.len();
/// let ret = unsafe { nc::epoll_wait(epfd, &mut events, events_len as i32, 0) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(1));
///
/// for event in &events {
///     // Ready to read
///     if event.events == nc::EPOLLIN {
///         let ready_fd = unsafe { event.data.fd };
///         assert_eq!(ready_fd, fds[0]);
///         let mut buf = vec![0_u8; 64];
///         let buf_len = buf.len();
///         let ret = unsafe { nc::read(ready_fd, buf.as_mut_ptr() as usize, buf_len) };
///         assert!(ret.is_ok());
///         let n_read = ret.unwrap() as usize;
///         assert_eq!(msg.as_bytes(), &buf[..n_read]);
///     }
/// }
///
/// let ret = unsafe { nc::close(fds[0]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[1]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(epfd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn epoll_wait(
    epfd: i32,
    events: &mut [epoll_event_t],
    max_events: i32,
    timeout: i32,
) -> Result<i32, Errno> {
    let epfd = epfd as usize;
    let events_ptr = events.as_mut_ptr() as usize;
    let max_events = max_events as usize;
    let timeout = timeout as usize;
    syscall4(SYS_EPOLL_WAIT, epfd, events_ptr, max_events, timeout).map(|ret| ret as i32)
}

/// Create a file descriptor for event notification.
pub unsafe fn eventfd(count: u32) -> Result<i32, Errno> {
    let count = count as usize;
    syscall1(SYS_EVENTFD, count).map(|ret| ret as i32)
}

/// Create a file descriptor for event notification.
pub unsafe fn eventfd2(count: u32, flags: i32) -> Result<i32, Errno> {
    let count = count as usize;
    let flags = flags as usize;
    syscall2(SYS_EVENTFD2, count, flags).map(|ret| ret as i32)
}

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
    let argv_ptr = to_c_str_vec(argv).as_ptr() as usize;
    let env_ptr = to_c_str_vec(env).as_ptr() as usize;
    syscall3(SYS_EXECVE, filename_ptr, argv_ptr, env_ptr).map(drop)
}

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

/// Terminate current process.
///
/// # Example
///
/// ```
/// unsafe { nc::exit(0); }
/// ```
pub unsafe fn exit(status: i32) -> ! {
    let status = status as usize;
    let _ret = syscall1(SYS_EXIT, status);
    unreachable!();
}

/// Exit all threads in a process's thread group.
///
/// # Example
///
/// ```
/// unsafe { nc::exit_group(0); }
/// ```
pub unsafe fn exit_group(status: i32) -> ! {
    let status = status as usize;
    let _ret = syscall1(SYS_EXIT_GROUP, status);
    unreachable!();
}

/// Check user's permission for a file.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::faccessat(nc::AT_FDCWD, "/etc/passwd", nc::F_OK) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn faccessat<P: AsRef<Path>>(dfd: i32, filename: P, mode: i32) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall3(SYS_FACCESSAT, dfd, filename_ptr, mode).map(drop)
}

/// Check user's permission for a file.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::faccessat2(nc::AT_FDCWD, "/etc/passwd", nc::F_OK, nc::AT_SYMLINK_NOFOLLOW) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn faccessat2<P: AsRef<Path>>(
    dfd: i32,
    filename: P,
    mode: i32,
    flags: i32,
) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    let flags = flags as usize;
    syscall4(SYS_FACCESSAT2, dfd, filename_ptr, mode, flags).map(drop)
}

/// Predeclare an access pattern for file data.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::fadvise64(fd, 0, 1024, nc::POSIX_FADV_NORMAL) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fadvise64(fd: i32, offset: loff_t, len: size_t, advice: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    let advice = advice as usize;
    syscall4(SYS_FADVISE64, fd, offset, len, advice).map(drop)
}

/// Predeclare an access pattern for file data.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let offset = 0;
/// let len = 4 * 1024;
/// let ret = unsafe { nc::fadvise64_64(fd, offset, len, nc::POSIX_FADV_NORMAL) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fadvise64_64(fd: i32, offset: loff_t, len: loff_t, advice: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    let len = len as usize;
    let advice = advice as usize;
    syscall4(SYS_FADVISE64_64, fd, offset, len, advice).map(drop)
}

/// Manipulate file space.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fallocate";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::fallocate(fd, 0, 0, 64 * 1024) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fallocate(fd: i32, mode: i32, offset: loff_t, len: loff_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let mode = mode as usize;
    let offset = offset as usize;
    let len = len as usize;
    syscall4(SYS_FALLOCATE, fd, mode, offset, len).map(drop)
}

/// Create and initialize fanotify group.
pub unsafe fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, Errno> {
    let flags = flags as usize;
    let event_f_flags = event_f_flags as usize;
    syscall2(SYS_FANOTIFY_INIT, flags, event_f_flags).map(|ret| ret as i32)
}

/// Add, remove, or modify an fanotify mark on a filesystem object
pub unsafe fn fanotify_mark<P: AsRef<Path>>(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    fd: i32,
    filename: P,
) -> Result<(), Errno> {
    let fanotify_fd = fanotify_fd as usize;
    let flags = flags as usize;
    let mask = mask as usize;
    let fd = fd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall5(
        SYS_FANOTIFY_MARK,
        fanotify_fd,
        flags,
        mask,
        fd,
        filename_ptr,
    )
    .map(drop)
}

/// Change working directory.
///
/// # Example
///
/// ```
/// let path = "/tmp";
/// // Open folder directly.
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_PATH, 0) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::fchdir(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fchdir(fd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    syscall1(SYS_FCHDIR, fd).map(drop)
}

/// Change permissions of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-fchmod";
/// let fd = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::fchmod(fd, 0o600) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fchmod(fd: i32, mode: mode_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let mode = mode as usize;
    syscall2(SYS_FCHMOD, fd, mode).map(drop)
}

/// Change permissions of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-fchmodat";
/// let fd = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::fchmodat(nc::AT_FDCWD, filename, 0o600) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fchmodat<P: AsRef<Path>>(dirfd: i32, filename: P, mode: mode_t) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall3(SYS_FCHMODAT, dirfd, filename_ptr, mode).map(drop)
}

/// Change ownership of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-fchown";
/// let fd = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::fchown(fd, 0, 0) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fchown(fd: i32, user: uid_t, group: gid_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let user = user as usize;
    let group = group as usize;
    syscall3(SYS_FCHOWN, fd, user, group).map(drop)
}

/// Change ownership of a file.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-fchown";
/// let fd = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::fchownat(nc::AT_FDCWD, filename, 0, 0, 0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename,0 ) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fchownat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    user: uid_t,
    group: gid_t,
    flag: i32,
) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let user = user as usize;
    let group = group as usize;
    let flag = flag as usize;
    syscall5(SYS_FCHOWNAT, dirfd, filename_ptr, user, group, flag).map(drop)
}

/// manipulate file descriptor.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let ret = unsafe { nc::fcntl(fd, nc::F_DUPFD, 0) };
/// assert!(ret.is_ok());
/// let fd2 = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd2) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fcntl(fd: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    let fd = fd as usize;
    let cmd = cmd as usize;
    syscall3(SYS_FCNTL, fd, cmd, arg).map(|ret| ret as i32)
}

/// Manipulate file descriptor.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let ret = unsafe { nc::fcntl64(fd, nc::F_DUPFD, 0) };
/// assert!(ret.is_ok());
/// let fd2 = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd2) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fcntl64(fd: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    let fd = fd as usize;
    let cmd = cmd as usize;
    syscall3(SYS_FCNTL64, fd, cmd, arg).map(|ret| ret as i32)
}

/// Flush all modified in-core data (exclude metadata) refered by `fd` to disk.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fdatasync";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let msg = b"Hello, Rust";
/// let ret = unsafe { nc::write(fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(msg.len() as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fdatasync(fd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    syscall1(SYS_FDATASYNC, fd).map(drop)
}

/// Get extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fgetxattr";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::fgetxattr(fd, attr_name, buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(attr_value.len() as nc::ssize_t));
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(attr_value.as_bytes(), &buf[..attr_len]);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fgetxattr<P: AsRef<Path>>(
    fd: i32,
    name: P,
    value: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall4(SYS_FGETXATTR, fd, name_ptr, value, size).map(|ret| ret as ssize_t)
}

/// Load a kernel module.
pub unsafe fn finit_module<P: AsRef<Path>>(
    fd: i32,
    param_values: P,
    flags: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let param_values = PathBuf::new(param_values);
    let param_values_ptr = param_values.as_ptr() as usize;
    let flags = flags as usize;
    syscall3(SYS_FINIT_MODULE, fd, param_values_ptr, flags).map(drop)
}

/// List extended attribute names.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-flistxattr";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::flistxattr(fd, buf.as_mut_ptr() as usize, buf_len) };
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(&buf[..attr_len - 1], attr_name.as_bytes());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn flistxattr(fd: i32, list: usize, size: size_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    syscall3(SYS_FLISTXATTR, fd, list, size).map(|ret| ret as ssize_t)
}

/// Apply or remove an advisory lock on an open file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-flock";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::flock(fd, nc::LOCK_EX) };
/// assert!(ret.is_ok());
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(msg.len() as nc::ssize_t));
/// let ret = unsafe { nc::flock(fd, nc::LOCK_UN) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path,0 ) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn flock(fd: i32, operation: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let operation = operation as usize;
    syscall2(SYS_FLOCK, fd, operation).map(drop)
}

/// Create a child process.
///
/// # Example
///
/// ```
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
/// ```
pub unsafe fn fork() -> Result<pid_t, Errno> {
    syscall0(SYS_FORK).map(|ret| ret as pid_t)
}

/// Remove an extended attribute.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fremovexattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::fremovexattr(fd, attr_name) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fremovexattr<P: AsRef<Path>>(fd: i32, name: P) -> Result<(), Errno> {
    let fd = fd as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall2(SYS_FREMOVEXATTR, fd, name_ptr).map(drop)
}

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

/// Set extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fsetxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::fsetxattr(
///         fd,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fsetxattr<P: AsRef<Path>>(
    fd: i32,
    name: P,
    value: usize,
    size: size_t,
    flags: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(SYS_FSETXATTR, fd, name_ptr, value, size, flags).map(drop)
}

/// Create a kernel mount representation for a new, prepared superblock.
pub unsafe fn fsmount(fs_fd: i32, flags: u32, attr_flags: u32) -> Result<i32, Errno> {
    let fs_fd = fs_fd as usize;
    let flags = flags as usize;
    let attr_flags = attr_flags as usize;
    syscall3(SYS_FSMOUNT, fs_fd, flags, attr_flags).map(|ret| ret as i32)
}

/// Open a filesystem by name so that it can be configured for mounting.
pub unsafe fn fsopen<P: AsRef<Path>>(fs_name: P, flags: u32) -> Result<(), Errno> {
    let fs_name = PathBuf::new(fs_name);
    let fs_name_ptr = fs_name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_FSOPEN, fs_name_ptr, flags).map(drop)
}

/// Pick a superblock into a context for reconfiguration.
pub unsafe fn fspick<P: AsRef<Path>>(dfd: i32, path: P, flags: i32) -> Result<i32, Errno> {
    let dfd = dfd as usize;
    let path = PathBuf::new(path);
    let path_ptr = path.as_ptr() as usize;
    let flags = flags as usize;
    syscall3(SYS_FSPICK, dfd, path_ptr, flags).map(|ret| ret as i32)
}

/// Get file status about a file descriptor.
///
/// # example
///
/// ```
/// let path = "/tmp";
/// // Open folder directly.
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_PATH, 0) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let mut stat = nc::stat_t::default();
/// let ret = unsafe { nc::fstat(fd, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a directory.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFDIR);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fstat(fd: i32, statbuf: &mut stat_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let statbuf_ptr = statbuf as *mut stat_t as usize;
    syscall2(SYS_FSTAT, fd, statbuf_ptr).map(drop)
}

/// Get file status.
///
/// # Example
///
/// ```
/// let path = "/tmp";
/// // Open folder directly.
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_PATH, 0) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let mut stat = nc::stat_t::default();
/// let ret = unsafe { nc::fstat63(fd, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a directory.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFDIR);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fstat64(fd: i32, statbuf: &mut stat64_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let statbuf_ptr = statbuf as *mut stat64_t as usize;
    syscall2(SYS_FSTAT64, fd, statbuf_ptr).map(drop)
}

/// Get file status.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat64_t::default();
/// let ret = unsafe { nc::fstatat64(nc::AT_FDCWD, path, &mut stat, nc::AT_SYMLINK_NOFOLLOW) };
/// assert!(ret.is_ok());
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn fstatat64<P: AsRef<Path>>(
    dfd: i32,
    filename: P,
    statbuf: &mut stat64_t,
    flag: i32,
) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat64_t as usize;
    let flag = flag as usize;
    syscall4(SYS_FSTATAT64, dfd, filename_ptr, statbuf_ptr, flag).map(drop)
}

/// Get filesystem statistics.
///
/// # Example
///
/// ```
/// let path = "/usr";
/// // Open folder directly.
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_PATH, 0) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let mut statfs = nc::statfs_t::default();
/// let ret = unsafe { nc::fstatfs(fd, &mut statfs) };
/// assert!(ret.is_ok());
/// assert!(statfs.f_bfree > 0);
/// assert!(statfs.f_bavail > 0);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fstatfs(fd: i32, buf: &mut statfs_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let buf_ptr = buf as *mut statfs_t as usize;
    syscall2(SYS_FSTATFS, fd, buf_ptr).map(drop)
}

/// Get filesystem statistics.
///
/// # Example
///
/// ```
/// let path = "/usr";
/// // Open folder directly.
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_PATH, 0) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let mut statfs = nc::statfs64_t::default();
/// let ret = unsafe { nc::fstatfs64(fd, &mut statfs) };
/// assert!(ret.is_ok());
/// assert!(statfs.f_bfree > 0);
/// assert!(statfs.f_bavail > 0);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fstatfs64(fd: i32, buf: &mut statfs64_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let buf_ptr = buf as *mut statfs64_t as usize;
    syscall2(SYS_FSTATFS64, fd, buf_ptr).map(drop)
}

/// Flush all modified in-core data refered by `fd` to disk.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-fsync";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_CREAT | nc::O_WRONLY, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let buf = b"Hello, Rust";
/// let n_write = unsafe { nc::write(fd, buf.as_ptr() as usize, buf.len()) };
/// assert_eq!(n_write, Ok(buf.len() as isize));
/// let ret = unsafe { nc::fsync(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn fsync(fd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    syscall1(SYS_FSYNC, fd).map(drop)
}

/// Truncate an opened file to a specified length.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-ftruncate";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::ftruncate(fd, 64 * 1024) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn ftruncate(fd: i32, length: off_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let length = length as usize;
    syscall2(SYS_FTRUNCATE, fd, length).map(drop)
}

/// Truncate a file to a specific length.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-ftruncate64";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::ftruncate64(fd, 64 * 1024) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn ftruncate64(fd: i32, len: loff_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let len = len as usize;
    syscall2(SYS_FTRUNCATE64, fd, len).map(drop)
}

/// Fast user-space locking.
pub unsafe fn futex(
    uaddr: &mut i32,
    futex_op: i32,
    val: u32,
    timeout: &mut timespec_t,
    uaddr2: &mut i32,
    val3: i32,
) -> Result<i32, Errno> {
    let uaddr_ptr = uaddr as *mut i32 as usize;
    let futex_op = futex_op as usize;
    let val = val as usize;
    let timeout_ptr = timeout as *mut timespec_t as usize;
    let uaddr2_ptr = uaddr2 as *mut i32 as usize;
    let val3 = val3 as usize;
    syscall6(
        SYS_FUTEX,
        uaddr_ptr,
        futex_op,
        val,
        timeout_ptr,
        uaddr2_ptr,
        val3,
    )
    .map(|ret| ret as i32)
}

/// Change timestamp of a file relative to a directory file discriptor.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-futimesat";
/// let fd = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(fd.is_ok());
/// let fd = fd.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let times = [
///     nc::timeval_t {
///         tv_sec: 100,
///         tv_usec: 0,
///     },
///     nc::timeval_t {
///         tv_sec: 10,
///         tv_usec: 0,
///     },
/// ];
/// let ret = unsafe { nc::futimesat(nc::AT_FDCWD, path, &times) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn futimesat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    times: &[timeval_t; 2],
) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let times_ptr = times.as_ptr() as usize;
    syscall3(SYS_FUTIMESAT, dirfd, filename_ptr, times_ptr).map(drop)
}

/// Determine CPU and NUMA node on which the calling thread is running.
///
/// # Example
///
/// ```
/// let mut cpu = 0;
/// let mut node = 0;
/// let mut cache = nc::getcpu_cache_t::default();
/// let ret = unsafe { nc::getcpu(&mut cpu, &mut node, &mut cache) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getcpu(
    cpu: &mut u32,
    node: &mut u32,
    cache: &mut getcpu_cache_t,
) -> Result<(), Errno> {
    let cpu_ptr = cpu as *mut u32 as usize;
    let node_ptr = node as *mut u32 as usize;
    let cache_ptr = cache as *mut getcpu_cache_t as usize;
    syscall3(SYS_GETCPU, cpu_ptr, node_ptr, cache_ptr).map(drop)
}

/// Get current working directory.
///
/// # Example
///
/// ```
/// let mut buf = [0_u8; nc::PATH_MAX as usize + 1];
/// let ret = unsafe { nc::getcwd(buf.as_mut_ptr() as usize, buf.len()) };
/// assert!(ret.is_ok());
/// // Remove null-terminal char.
/// let path_len = ret.unwrap() as usize - 1;
/// let cwd = std::str::from_utf8(&buf[..path_len]);
/// assert!(cwd.is_ok());
/// ```
pub unsafe fn getcwd(buf: usize, size: size_t) -> Result<ssize_t, Errno> {
    syscall2(SYS_GETCWD, buf, size).map(|ret| ret as ssize_t)
}

/// Get directory entries.
///
/// # Example
///
/// ```
/// let path = "/etc";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_DIRECTORY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// const BUF_SIZE: usize = 4 * 1024;
/// loop {
///     // TODO(Shaohua): Only allocate one buf block.
///     let mut buf: Vec<u8> = vec![0; BUF_SIZE];
///     let ret = unsafe { nc::getdents(fd, buf.as_mut_ptr() as usize, BUF_SIZE) };
///     assert!(ret.is_ok());
///
///     let buf_box = buf.into_boxed_slice();
///     let buf_box_ptr = Box::into_raw(buf_box) as *mut u8 as usize;
///     let nread = ret.unwrap() as usize;
///     if nread == 0 {
///         break;
///     }
///
///     let mut bpos: usize = 0;
///     while bpos < nread {
///         let d = (buf_box_ptr + bpos) as *mut nc::linux_dirent_t;
///         let d_ref = unsafe { &(*d) };
///         let mut name_vec: Vec<u8> = vec![];
///         // TODO(Shaohua): Calculate string len of name.
///         for i in 0..nc::PATH_MAX {
///             let c = d_ref.d_name[i as usize];
///             if c == 0 {
///                 break;
///             }
///             name_vec.push(c);
///         }
///         let _name = String::from_utf8(name_vec).unwrap();
///
///         bpos += d_ref.d_reclen as usize;
///     }
/// }
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getdents(fd: i32, dirp: usize, count: size_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    syscall3(SYS_GETDENTS, fd, dirp, count).map(|ret| ret as ssize_t)
}

/// Get directory entries.
///
/// # Example
///
/// ```
/// let path = "/etc";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_DIRECTORY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// const BUF_SIZE: usize = 4 * 1024;
/// loop {
///     // TODO(Shaohua): Only allocate one buf block.
///     let mut buf: Vec<u8> = vec![0; BUF_SIZE];
///     let ret = unsafe { nc::getdents64(fd, buf.as_mut_ptr() as usize, BUF_SIZE) };
///     assert!(ret.is_ok());
///
///     let buf_box = buf.into_boxed_slice();
///     let buf_box_ptr = Box::into_raw(buf_box) as *mut u8 as usize;
///     let nread = ret.unwrap() as usize;
///     if nread == 0 {
///         break;
///     }
///
///     let mut bpos: usize = 0;
///     while bpos < nread {
///         let d = (buf_box_ptr + bpos) as *mut nc::linux_dirent64_t;
///         let d_ref = unsafe { &(*d) };
///         let mut name_vec: Vec<u8> = vec![];
///         // TODO(Shaohua): Calculate string len of name.
///         for i in 0..nc::PATH_MAX {
///             let c = d_ref.d_name[i as usize];
///             if c == 0 {
///                 break;
///             }
///             name_vec.push(c);
///         }
///         let _name = String::from_utf8(name_vec).unwrap();
///
///         bpos += d_ref.d_reclen as usize;
///     }
/// }
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getdents64(fd: i32, dirp: usize, count: size_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    syscall3(SYS_GETDENTS64, fd, dirp, count).map(|ret| ret as ssize_t)
}

/// Get the effective group ID of the calling process.
///
/// # Example
///
/// ```
/// let egid = unsafe { nc::getegid() };
/// assert!(egid > 0);
/// ```
#[must_use]
pub unsafe fn getegid() -> gid_t {
    // This function is always successful.
    syscall0(SYS_GETEGID).expect("getegid() failed") as gid_t
}

/// Get the effective user ID of the calling process.
///
/// # Example
///
/// ```
/// let euid = unsafe { nc::geteuid() };
/// assert!(euid > 0);
/// ```
#[must_use]
pub unsafe fn geteuid() -> uid_t {
    // This function is always successful.
    syscall0(SYS_GETEUID).expect("geteuid() failed") as uid_t
}

/// Get the real group ID of the calling process.
///
/// # Example
///
/// ```
/// let gid = unsafe { nc::getgid() };
/// assert!(gid > 0);
/// ```
#[must_use]
pub unsafe fn getgid() -> gid_t {
    // This function is always successful.
    syscall0(SYS_GETGID).expect("getgid() failed") as gid_t
}

/// Get list of supplementary group Ids.
///
/// # Example
///
/// ```
/// let mut groups = vec![];
/// let ret = unsafe { nc::getgroups(0, &mut groups) };
/// assert!(ret.is_ok());
/// let total_num = ret.unwrap();
/// groups.resize(total_num as usize, 0);
///
/// let ret = unsafe { nc::getgroups(total_num, &mut groups) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(total_num));
/// ```
pub unsafe fn getgroups(size: i32, group_list: &mut [gid_t]) -> Result<i32, Errno> {
    let size = size as usize;
    let group_ptr = group_list.as_mut_ptr() as usize;
    syscall2(SYS_GETGROUPS, size, group_ptr).map(|ret| ret as i32)
}

/// Get value of an interval timer.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
///     let msg = "Hello alarm";
///     let _ = unsafe { nc::write(2, msg.as_ptr() as usize, msg.len()) };
/// }
///
/// let sa = nc::sigaction_t {
///     sa_handler: handle_alarm as nc::sighandler_t,
///     sa_flags: 0,
///     ..nc::sigaction_t::default()
/// };
/// let mut old_sa = nc::sigaction_t::default();
/// let ret = unsafe { nc::rt_sigaction(nc::SIGALRM, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
/// assert!(ret.is_ok());
///
/// // Single shot timer, actived after 1 second.
/// let itv = nc::itimerval_t {
///     it_value: nc::timeval_t {
///         tv_sec: 1,
///         tv_usec: 0,
///     },
///     it_interval: nc::timeval_t {
///         tv_sec: 0,
///         tv_usec: 0,
///     },
/// };
/// let mut prev_itv = nc::itimerval_t::default();
/// let ret = unsafe { nc::setitimer(nc::ITIMER_REAL, &itv, &mut prev_itv) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::getitimer(nc::ITIMER_REAL, &mut prev_itv) };
/// assert!(ret.is_ok());
/// assert!(prev_itv.it_value.tv_sec <= itv.it_value.tv_sec);
///
/// let mask = nc::sigset_t::default();
/// let _ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///
/// let ret = unsafe { nc::getitimer(nc::ITIMER_REAL, &mut prev_itv) };
/// assert!(ret.is_ok());
/// assert_eq!(prev_itv.it_value.tv_sec, 0);
/// assert_eq!(prev_itv.it_value.tv_usec, 0);
/// ```
pub unsafe fn getitimer(which: i32, curr_val: &mut itimerval_t) -> Result<(), Errno> {
    let which = which as usize;
    let curr_val_ptr = curr_val as *mut itimerval_t as usize;
    syscall2(SYS_GETITIMER, which, curr_val_ptr).map(drop)
}

/// Get name of connected peer socket.
pub unsafe fn getpeername(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let addr_ptr = addr as *mut sockaddr_in_t as usize;
    let addrlen_ptr = addrlen as *mut socklen_t as usize;
    syscall3(SYS_GETPEERNAME, sockfd, addr_ptr, addrlen_ptr).map(drop)
}

/// Returns the PGID(process group ID) of the process specified by `pid`.
///
/// # Example
///
/// ```
/// let ppid = unsafe { nc::getppid() };
/// let pgid = unsafe { nc::getpgid(ppid) };
/// assert!(pgid.is_ok());
/// ```
pub unsafe fn getpgid(pid: pid_t) -> Result<pid_t, Errno> {
    let pid = pid as usize;
    syscall1(SYS_GETPGID, pid).map(|ret| ret as pid_t)
}

/// Get the process group ID of the calling process.
///
/// # Example
///
/// ```
/// let pgroup = unsafe { nc::getpgrp() };
/// assert!(pgroup > 0);
/// ```
#[must_use]
pub unsafe fn getpgrp() -> pid_t {
    // This function is always successful.
    syscall0(SYS_GETPGRP).expect("getpgrp() failed") as pid_t
}

/// Get the process ID (PID) of the calling process.
///
/// # Example
///
/// ```
/// let pid = unsafe { nc::getpid() };
/// assert!(pid > 0);
/// ```
#[must_use]
pub unsafe fn getpid() -> pid_t {
    // This function is always successful.
    syscall0(SYS_GETPID).expect("getpid() failed") as pid_t
}

/// Get the process ID of the parent of the calling process.
///
/// # Example
///
/// ```
/// let ppid = unsafe { nc::getppid() };
/// assert!(ppid > 0);
/// ```
#[must_use]
pub unsafe fn getppid() -> pid_t {
    // This function is always successful.
    syscall0(SYS_GETPPID).expect("getppid() failed") as pid_t
}

/// Get program scheduling priority.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::getpriority(nc::PRIO_PROCESS, nc::getpid()) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getpriority(which: i32, who: i32) -> Result<i32, Errno> {
    let which = which as usize;
    let who = who as usize;
    syscall2(SYS_GETPRIORITY, which, who).map(|ret| {
        let ret = ret as i32;
        if ret > PRIO_MAX {
            return PRIO_MAX - ret;
        }
        ret
    })
}

/// Obtain a series of random bytes.
///
/// # Example
///
/// ```
/// let mut buf = [0_u8; 32];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::getrandom(&mut buf, buf_len, 0) };
/// assert!(ret.is_ok());
/// let size = ret.unwrap() as usize;
/// assert!(size <= buf_len);
/// ```
pub unsafe fn getrandom(buf: &mut [u8], buf_len: usize, flags: u32) -> Result<ssize_t, Errno> {
    let buf_ptr = buf.as_mut_ptr() as usize;
    let flags = flags as usize;
    syscall3(SYS_GETRANDOM, buf_ptr, buf_len, flags).map(|ret| ret as ssize_t)
}

/// Get real, effect and saved group ID.
///
/// # Example
///
/// ```
/// let mut rgid = 0;
/// let mut egid = 0;
/// let mut sgid = 0;
/// let ret = unsafe { nc::getresgid(&mut rgid, &mut egid, &mut sgid) };
/// assert!(ret.is_ok());
/// assert!(rgid > 0);
/// assert!(egid > 0);
/// assert!(sgid > 0);
/// ```
pub unsafe fn getresgid(rgid: &mut gid_t, egid: &mut gid_t, sgid: &mut gid_t) -> Result<(), Errno> {
    let rgid_ptr = rgid as *mut gid_t as usize;
    let egid_ptr = egid as *mut gid_t as usize;
    let sgid_ptr = sgid as *mut gid_t as usize;
    syscall3(SYS_GETRESGID, rgid_ptr, egid_ptr, sgid_ptr).map(drop)
}

/// Get real, effect and saved user ID.
///
/// # Example
///
/// ```
/// let mut ruid = 0;
/// let mut euid = 0;
/// let mut suid = 0;
/// let ret = unsafe { nc::getresuid(&mut ruid, &mut euid, &mut suid) };
/// assert!(ret.is_ok());
/// assert!(ruid > 0);
/// assert!(euid > 0);
/// assert!(suid > 0);
/// ```
pub unsafe fn getresuid(ruid: &mut uid_t, euid: &mut uid_t, suid: &mut uid_t) -> Result<(), Errno> {
    let ruid_ptr = ruid as *mut uid_t as usize;
    let euid_ptr = euid as *mut uid_t as usize;
    let suid_ptr = suid as *mut uid_t as usize;
    syscall3(SYS_GETRESUID, ruid_ptr, euid_ptr, suid_ptr).map(drop)
}

/// Get resource limit.
///
/// # Example
///
/// ```
/// let mut rlimit = nc::rlimit_t::default();
/// let ret = unsafe { nc::getrlimit(nc::RLIMIT_NOFILE, &mut rlimit) };
/// assert!(ret.is_ok());
/// assert!(rlimit.rlim_cur > 0);
/// assert!(rlimit.rlim_max > 0);
/// ```
pub unsafe fn getrlimit(resource: i32, rlim: &mut rlimit_t) -> Result<(), Errno> {
    let resource = resource as usize;
    let rlim_ptr = rlim as *mut rlimit_t as usize;
    syscall2(SYS_GETRLIMIT, resource, rlim_ptr).map(drop)
}

/// Get resource usage.
///
/// # Example
///
/// ```
/// let mut usage = nc::rusage_t::default();
/// let ret = unsafe { nc::getrusage(nc::RUSAGE_SELF, &mut usage) };
/// assert!(ret.is_ok());
/// assert!(usage.ru_maxrss > 0);
/// assert_eq!(usage.ru_nswap, 0);
/// ```
pub unsafe fn getrusage(who: i32, usage: &mut rusage_t) -> Result<(), Errno> {
    let who = who as usize;
    let usage_ptr = usage as *mut rusage_t as usize;
    syscall2(SYS_GETRUSAGE, who, usage_ptr).map(drop)
}

/// Get session Id.
///
/// # Example
///
/// ```
/// let ppid = unsafe { nc::getppid() };
/// let sid = unsafe { nc::getsid(ppid) };
/// assert!(sid > 0);
/// ```
#[must_use]
pub unsafe fn getsid(pid: pid_t) -> pid_t {
    let pid = pid as usize;
    // This function is always successful.
    syscall1(SYS_GETSID, pid).expect("getsid() failed") as pid_t
}

/// Get current address to which the socket `sockfd` is bound.
pub unsafe fn getsockname(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let addr_ptr = addr as *mut sockaddr_in_t as usize;
    let addrlen_ptr = addrlen as *mut socklen_t as usize;
    syscall3(SYS_GETSOCKNAME, sockfd, addr_ptr, addrlen_ptr).map(drop)
}

/// Get options on sockets
pub unsafe fn getsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: &mut usize,
    optlen: &mut socklen_t,
) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let level = level as usize;
    let optname = optname as usize;
    let optval_ptr = optval as *mut usize as usize;
    let optlen_ptr = optlen as *mut socklen_t as usize;
    syscall5(
        SYS_GETSOCKOPT,
        sockfd,
        level,
        optname,
        optval_ptr,
        optlen_ptr,
    )
    .map(drop)
}

/// Get the caller's thread ID (TID).
///
/// # Example
///
/// ```
/// let tid = unsafe { nc::gettid() };
/// assert!(tid > 0);
/// ```
#[must_use]
pub unsafe fn gettid() -> pid_t {
    // This function is always successful.
    syscall0(SYS_GETTID).expect("getpid() failed") as pid_t
}

/// Get time.
///
/// # Example
///
/// ```
/// let mut tv = nc::timeval_t::default();
/// let mut tz = nc::timezone_t::default();
/// let ret = unsafe { nc::gettimeofday(&mut tv, &mut tz) };
/// assert!(ret.is_ok());
/// assert!(tv.tv_sec > 1611380386);
/// ```
pub unsafe fn gettimeofday(timeval: &mut timeval_t, tz: &mut timezone_t) -> Result<(), Errno> {
    let timeval_ptr = timeval as *mut timeval_t as usize;
    let tz_ptr = tz as *mut timezone_t as usize;
    syscall2(SYS_GETTIMEOFDAY, timeval_ptr, tz_ptr).map(drop)
}

/// Get the real user ID of the calling process.
///
/// # Example
///
/// ```
/// let uid = unsafe { nc::getuid() };
/// assert!(uid > 0);
/// ```
#[must_use]
pub unsafe fn getuid() -> uid_t {
    // This function is always successful.
    syscall0(SYS_GETUID).expect("getuid() failed") as uid_t
}

/// Get extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-getxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::getxattr(path, attr_name, buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(attr_value.len() as nc::ssize_t));
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(attr_value.as_bytes(), &buf[..attr_len]);
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getxattr<P: AsRef<Path>>(
    filename: P,
    name: P,
    value: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall4(SYS_GETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
}

/// Retrieve NUMA memory policy for a thread
pub unsafe fn get_mempolicy(
    mode: &mut i32,
    nmask: &mut usize,
    maxnode: usize,
    addr: usize,
    flags: usize,
) -> Result<(), Errno> {
    let mode_ptr = mode as *mut i32 as usize;
    let nmask_ptr = nmask as *mut usize as usize;
    syscall5(SYS_GET_MEMPOLICY, mode_ptr, nmask_ptr, maxnode, addr, flags).map(drop)
}

/// Get list of robust futexes.
// TODO(Shaohua): Fix argument type.
pub unsafe fn get_robust_list(
    pid: pid_t,
    head_ptr: &mut usize,
    len_ptr: &mut size_t,
) -> Result<(), Errno> {
    let pid = pid as usize;
    let head_ptr = head_ptr as *mut usize as usize;
    let len_ptr = len_ptr as *mut size_t as usize;
    syscall3(SYS_GET_ROBUST_LIST, pid, head_ptr, len_ptr).map(drop)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
/// Get thread-local storage information.
pub unsafe fn get_thread_area(user_desc: &mut user_desc_t) -> Result<(), Errno> {
    let user_desc_ptr = user_desc as *mut user_desc_t as usize;
    syscall1(SYS_GET_THREAD_AREA, user_desc_ptr).map(drop)
}

/// Make process 0 idle.
///
/// Never returns for process 0, and already returns EPERM for a user process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::idle() };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn idle() -> Result<(), Errno> {
    syscall0(SYS_IDLE).map(drop)
}

/// Load a kernel module.
pub unsafe fn init_module<P: AsRef<Path>>(
    module_image: usize,
    len: usize,
    param_values: P,
) -> Result<(), Errno> {
    let param_values = PathBuf::new(param_values);
    let param_values_ptr = param_values.as_ptr() as usize;
    syscall3(SYS_INIT_MODULE, module_image, len, param_values_ptr).map(drop)
}

/// Add a watch to an initialized inotify instance.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::inotify_init1(nc::IN_NONBLOCK | nc::IN_CLOEXEC) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::inotify_add_watch(fd, path, nc::IN_MODIFY) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn inotify_add_watch<P: AsRef<Path>>(
    fd: i32,
    filename: P,
    mask: u32,
) -> Result<i32, Errno> {
    let fd = fd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mask = mask as usize;
    syscall3(SYS_INOTIFY_ADD_WATCH, fd, filename_ptr, mask).map(|ret| ret as i32)
}

/// Initialize an inotify instance.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::inotify_init() };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn inotify_init() -> Result<i32, Errno> {
    syscall0(SYS_INOTIFY_INIT).map(|ret| ret as i32)
}

/// Initialize an inotify instance.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::inotify_init1(nc::IN_NONBLOCK | nc::IN_CLOEXEC) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn inotify_init1(flags: i32) -> Result<i32, Errno> {
    let flags = flags as usize;
    syscall1(SYS_INOTIFY_INIT1, flags).map(|ret| ret as i32)
}

/// Remove an existing watch from an inotify instance.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::inotify_init1(nc::IN_NONBLOCK | nc::IN_CLOEXEC) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::inotify_add_watch(fd, path, nc::IN_MODIFY) };
/// assert!(ret.is_ok());
/// let wd = ret.unwrap();
/// let ret = unsafe { nc::inotify_rm_watch(fd, wd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn inotify_rm_watch(fd: i32, wd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let wd = wd as usize;
    syscall2(SYS_INOTIFY_RM_WATCH, fd, wd).map(drop)
}

/// Control device.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-ioctl";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut attr: i32 = 0;
/// let cmd = -2146933247; // nc::FS_IOC_GETFLAGS
/// let ret = unsafe { nc::ioctl(fd, cmd, &mut attr as *mut i32 as usize) };
/// assert!(ret.is_ok());
/// println!("attr: {}", attr);
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn ioctl(fd: i32, cmd: i32, arg: usize) -> Result<(), Errno> {
    let fd = fd as usize;
    let cmd = cmd as usize;
    syscall3(SYS_IOCTL, fd, cmd, arg).map(drop)
}

/// Set port input/output permissions.
pub unsafe fn ioperm(from: usize, num: usize, turn_on: i32) -> Result<(), Errno> {
    let turn_on = turn_on as usize;
    syscall3(SYS_IOPERM, from, num, turn_on).map(drop)
}

/// Change I/O privilege level.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::iopl(1) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn iopl(level: i32) -> Result<(), Errno> {
    let level = level as usize;
    syscall1(SYS_IOPL, level).map(drop)
}

/// Get I/O scheduling class and priority.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::ioprio_get(nc::IOPRIO_WHO_PROCESS, nc::getpid()) };
/// assert!(ret.is_ok());
/// let prio = ret.unwrap();
/// let _prio_class = unsafe { nc::ioprio_prio_class(prio) };
/// let _prio_data = unsafe { nc::ioprio_prio_data(prio) };
/// ```
pub unsafe fn ioprio_get(which: i32, who: i32) -> Result<i32, Errno> {
    let which = which as usize;
    let who = who as usize;
    syscall2(SYS_IOPRIO_GET, which, who).map(|ret| ret as i32)
}

/// Set I/O scheduling class and priority.
///
/// See [ioprio](https://www.kernel.org/doc/Documentation/block/ioprio.txt)
///
/// # Example
///
/// ```
/// // Change priority to lowest.
/// let new_prio_data = 7;
/// let new_prio = unsafe { nc::ioprio_prio_value(nc::IOPRIO_CLASS_IDLE, new_prio_data) };
/// let ret = unsafe { nc::ioprio_set(nc::IOPRIO_WHO_PROCESS, 0, new_prio) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn ioprio_set(which: i32, who: i32, ioprio: i32) -> Result<(), Errno> {
    let which = which as usize;
    let who = who as usize;
    let ioprio = ioprio as usize;
    syscall3(SYS_IOPRIO_SET, which, who, ioprio).map(drop)
}

/// Attempts to cancel an iocb previously passed to `io_submit`.
///
/// If the operation is successfully cancelled, the resulting event is
/// copied into the memory pointed to by result without being placed
/// into the completion queue and 0 is returned.
///
///
/// # Errors
/// - May fail with `-EFAULT` if any of the data structures pointed to are invalid.
/// - May fail with `-EINVAL` if `aio_context` specified by `ctx_id` is invalid.
/// - May fail with `-EAGAIN` if the iocb specified was not cancelled.
/// - Will fail with `-ENOSYS` if not implemented.
pub unsafe fn io_cancel(
    ctx_id: aio_context_t,
    iocb: &mut iocb_t,
    result: &mut io_event_t,
) -> Result<(), Errno> {
    let iocb_ptr = iocb as *mut iocb_t as usize;
    let result_ptr = result as *mut io_event_t as usize;
    syscall3(SYS_IO_CANCEL, ctx_id, iocb_ptr, result_ptr).map(drop)
}

/// Destroy the `aio_context` specified.
///
/// May cancel any outstanding AIOs and block on completion.
///
/// Will fail with `-ENOSYS` if not implemented.
/// May fail with `-EINVAL` if the context pointed to is invalid.
pub unsafe fn io_destroy(ctx_id: aio_context_t) -> Result<(), Errno> {
    syscall1(SYS_IO_DESTROY, ctx_id).map(drop)
}

/// Attempts to read at least `min_nr` events and up to nr events from
/// the completion queue for the `aio_context` specified by `ctx_id`.
///
/// If it succeeds, the number of read events is returned.
///
/// # Errors
///
/// - May fail with `-EINVAL` if `ctx_id` is invalid, if `min_nr` is out of range,
/// if `nr` is out of range, if `timeout` is out of range.
/// - May fail with `-EFAULT` if any of the memory specified is invalid.
/// - May return 0 or < `min_nr` if the timeout specified by timeout has elapsed
/// before sufficient events are available, where timeout == NULL
/// specifies an infinite timeout. Note that the timeout pointed to by timeout is relative.
/// - Will fail with `-ENOSYS` if not implemented.
pub unsafe fn io_getevents(
    ctx_id: aio_context_t,
    min_nr: isize,
    nr: isize,
    events: &mut io_event_t,
    timeout: &mut timespec_t,
) -> Result<i32, Errno> {
    let min_nr = min_nr as usize;
    let nr = nr as usize;
    let events_ptr = events as *mut io_event_t as usize;
    let timeout_ptr = timeout as *mut timespec_t as usize;
    syscall5(
        SYS_IO_GETEVENTS,
        ctx_id,
        min_nr,
        nr,
        events_ptr,
        timeout_ptr,
    )
    .map(|ret| ret as i32)
}

/// read asynchronous I/O events from the completion queue
pub unsafe fn io_pgetevents(
    ctx_id: aio_context_t,
    min_nr: isize,
    nr: isize,
    events: &mut io_event_t,
    timeout: &mut timespec_t,
    usig: &aio_sigset_t,
) -> Result<i32, Errno> {
    let min_nr = min_nr as usize;
    let nr = nr as usize;
    let events_ptr = events as *mut io_event_t as usize;
    let timeout_ptr = timeout as *mut timespec_t as usize;
    let usig_ptr = usig as *const aio_sigset_t as usize;
    syscall6(
        SYS_IO_PGETEVENTS,
        ctx_id,
        min_nr,
        nr,
        events_ptr,
        timeout_ptr,
        usig_ptr,
    )
    .map(|ret| ret as i32)
}

/// Create an asynchronous I/O context.
///
/// Create an `aio_context` capable of receiving at least `nr_events`.
/// ctxp must not point to an `aio_context` that already exists, and
/// must be initialized to 0 prior to the call.
///
/// On successful creation of the `aio_context`, `*ctxp` is filled in with the resulting
/// handle.
///
/// # Errors
///
/// - May fail with `-EINVAL` if `*ctxp` is not initialized,
/// if the specified `nr_events` exceeds internal limits.
/// - May fail with `-EAGAIN` if the specified `nr_events` exceeds the user's limit
/// of available events.
/// - May fail with `-ENOMEM` if insufficient kernel resources are available.
/// - May fail with `-EFAULT` if an invalid pointer is passed for ctxp.
/// - Will fail with `-ENOSYS` if not implemented.
pub unsafe fn io_setup(nr_events: u32, ctx_id: &mut aio_context_t) -> Result<(), Errno> {
    let nr_events = nr_events as usize;
    let ctx_id_ptr = ctx_id as *mut aio_context_t as usize;
    syscall2(SYS_IO_SETUP, nr_events, ctx_id_ptr).map(drop)
}

/// Queue the nr iocbs pointed to by iocbpp for processing.
///
/// Returns the number of iocbs queued.
///
/// # Errors
///
/// - May return `-EINVAL` if the `aio_context` specified by `ctx_id` is invalid,
/// if `nr` is < 0, if the `iocb` at `*iocbpp[0]` is not properly initialized,
/// if the operation specified is invalid for the file descriptor in the `iocb`.
/// - May fail with `-EFAULT` if any of the data structures point to invalid data.
/// - May fail with `-EBADF` if the file descriptor specified in the first
/// `iocb` is invalid.
/// - May fail with `-EAGAIN` if insufficient resources are available to queue any iocbs.
/// - Will return 0 if nr is 0.
/// - Will fail with `-ENOSYS` if not implemented.
// TODO(Shaohua): type of iocbpp is struct iocb**
pub unsafe fn io_submit(ctx_id: aio_context_t, nr: isize, iocb: &mut iocb_t) -> Result<i32, Errno> {
    let nr = nr as usize;
    let iocb_ptr = iocb as *mut iocb_t as usize;
    syscall3(SYS_IO_SUBMIT, ctx_id, nr, iocb_ptr).map(|ret| ret as i32)
}

pub unsafe fn io_uring_enter(
    fd: i32,
    to_submit: u32,
    min_complete: u32,
    flags: u32,
    sig: &sigset_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    let fd = fd as usize;
    let to_submit = to_submit as usize;
    let min_complete = min_complete as usize;
    let flags = flags as usize;
    let sig_ptr = sig as *const sigset_t as usize;
    syscall6(
        SYS_IO_URING_ENTER,
        fd,
        to_submit,
        min_complete,
        flags,
        sig_ptr,
        sigsetsize,
    )
    .map(|ret| ret as i32)
}

pub unsafe fn io_uring_register(
    fd: i32,
    opcode: u32,
    arg: usize,
    nr_args: u32,
) -> Result<i32, Errno> {
    let fd = fd as usize;
    let opcode = opcode as usize;
    let nr_args = nr_args as usize;
    syscall4(SYS_IO_URING_REGISTER, fd, opcode, arg, nr_args).map(|ret| ret as i32)
}

pub unsafe fn io_uring_setup(entries: u32, params: &mut io_uring_params_t) -> Result<i32, Errno> {
    let entries = entries as usize;
    let params_ptr = params as *mut io_uring_params_t as usize;
    syscall2(SYS_IO_URING_SETUP, entries, params_ptr).map(|ret| ret as i32)
}

/// System V IPC system calls.
pub unsafe fn ipc(
    call: u32,
    first: i32,
    second: i32,
    third: i32,
    ptr: usize,
    fifth: isize,
) -> Result<(), Errno> {
    let call = call as usize;
    let first = first as usize;
    let second = second as usize;
    let third = third as usize;
    let fifth = fifth as usize;
    syscall6(SYS_IPC, call, first, second, third, ptr, fifth).map(drop)
}

/// Compare two processes to determine if they share a kernel resource.
pub unsafe fn kcmp(
    pid1: pid_t,
    pid2: pid_t,
    type_: i32,
    idx1: usize,
    idx2: usize,
) -> Result<i32, Errno> {
    let pid1 = pid1 as usize;
    let pid2 = pid2 as usize;
    let type_ = type_ as usize;
    syscall5(SYS_KCMP, pid1, pid2, type_, idx1, idx2).map(|ret| ret as i32)
}

/// Load a new kernel for later execution.
pub unsafe fn kexec_load(
    entry: usize,
    nr_segments: usize,
    segments: &mut kexec_segment_t,
    flags: usize,
) -> Result<(), Errno> {
    let segments_ptr = segments as *mut kexec_segment_t as usize;
    syscall4(SYS_KEXEC_LOAD, entry, nr_segments, segments_ptr, flags).map(drop)
}

/// Manipulate the kernel's key management facility.
pub unsafe fn keyctl(
    operation: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    let operation = operation as usize;
    syscall5(SYS_KEYCTL, operation, arg2, arg3, arg4, arg5)
}

/// Send signal to a process.
///
/// # Example
///
/// ```
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
/// if pid == 0 {
///     // child process.
///     let args = [""];
///     let env = [""];
///     let ret = unsafe { nc::execve("/usr/bin/yes", &args, &env) };
///     assert!(ret.is_ok());
/// } else {
///     // parent process.
///     let ret = unsafe { nc::kill(pid, nc::SIGTERM) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn kill(pid: pid_t, signal: i32) -> Result<(), Errno> {
    let pid = pid as usize;
    let signal = signal as usize;
    syscall2(SYS_KILL, pid, signal).map(drop)
}

/// Change ownership of a file. Does not deference symbolic link.
///
/// # Example
///
/// ```
/// let filename = "/tmp/nc-lchown";
/// let ret = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::lchown(filename, 0, 0) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn lchown<P: AsRef<Path>>(filename: P, user: uid_t, group: gid_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let user = user as usize;
    let group = group as usize;
    syscall3(SYS_LCHOWN, filename_ptr, user, group).map(drop)
}

/// Get extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-lgetxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::lgetxattr(path, attr_name, buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(attr_value.len() as nc::ssize_t));
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(attr_value.as_bytes(), &buf[..attr_len]);
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn lgetxattr<P: AsRef<Path>>(
    filename: P,
    name: P,
    value: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall4(SYS_LGETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
}

/// Make a new name for a file.
///
/// # Example
///
/// ```
/// let old_filename = "/tmp/nc-link-src";
/// let ret = unsafe {
///     nc::openat(
///         nc::AT_FDCWD,
///         old_filename,
///         nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///         0o644
///     )
/// };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_filename = "/tmp/nc-link-dst";
/// let ret = unsafe { nc::link(old_filename, new_filename) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, old_filename, 0) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn link<P: AsRef<Path>>(old_filename: P, new_filename: P) -> Result<(), Errno> {
    let old_filename = PathBuf::new(old_filename);
    let old_filename_ptr = old_filename.as_ptr() as usize;
    let new_filename = PathBuf::new(new_filename);
    let new_filename_ptr = new_filename.as_ptr() as usize;
    syscall2(SYS_LINK, old_filename_ptr, new_filename_ptr).map(drop)
}

/// Make a new name for a file.
///
/// # Example
///
/// ```
/// let old_filename = "/tmp/nc-linkat-src";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, old_filename, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_filename = "/tmp/nc-linkat-dst";
/// let flags = nc::AT_SYMLINK_FOLLOW;
/// let ret = unsafe { nc::linkat(nc::AT_FDCWD, old_filename, nc::AT_FDCWD,  new_filename, flags) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, old_filename, 0) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_filename, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn linkat<P: AsRef<Path>>(
    olddfd: i32,
    oldfilename: P,
    newdfd: i32,
    newfilename: P,
    flags: i32,
) -> Result<(), Errno> {
    let olddfd = olddfd as usize;
    let oldfilename = PathBuf::new(oldfilename);
    let oldfilename_ptr = oldfilename.as_ptr() as usize;
    let newdfd = newdfd as usize;
    let newfilename = PathBuf::new(newfilename);
    let newfilename_ptr = newfilename.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(
        SYS_LINKAT,
        olddfd,
        oldfilename_ptr,
        newdfd,
        newfilename_ptr,
        flags,
    )
    .map(drop)
}

/// Listen for connections on a socket.
pub unsafe fn listen(sockfd: i32, backlog: i32) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let backlog = backlog as usize;
    syscall2(SYS_LISTEN, sockfd, backlog).map(drop)
}

/// List extended attribute names.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-listxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::listxattr(path, buf.as_mut_ptr() as usize, buf_len) };
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(&buf[..attr_len - 1], attr_name.as_bytes());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn listxattr<P: AsRef<Path>>(
    filename: P,
    list: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall3(SYS_LISTXATTR, filename_ptr, list, size).map(|ret| ret as ssize_t)
}

/// List extended attribute names.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-llistxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; 16];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::llistxattr(path, buf.as_mut_ptr() as usize, buf_len) };
/// let attr_len = ret.unwrap() as usize;
/// assert_eq!(&buf[..attr_len - 1], attr_name.as_bytes());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn llistxattr<P: AsRef<Path>>(
    filename: P,
    list: usize,
    size: size_t,
) -> Result<ssize_t, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall3(SYS_LLISTXATTR, filename_ptr, list, size).map(|ret| ret as ssize_t)
}

/// Return a directory entry's path.
// TODO(Shaohua): Returns a string.
pub unsafe fn lookup_dcookie(cookie: u64, buf: &mut [u8]) -> Result<i32, Errno> {
    let cookie = cookie as usize;
    let buf_ptr = buf.as_mut_ptr() as usize;
    let buf_len = buf.len();
    syscall3(SYS_LOOKUP_DCOOKIE, cookie, buf_ptr, buf_len).map(|ret| ret as i32)
}

/// Remove an extended attribute.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-lremovexattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::lremovexattr(path, attr_name) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// ```
pub unsafe fn lremovexattr<P: AsRef<Path>>(filename: P, name: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall2(SYS_LREMOVEXATTR, filename_ptr, name_ptr).map(drop)
}

/// Reposition file offset.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::lseek(fd, 42, nc::SEEK_SET) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn lseek(fd: i32, offset: off_t, whence: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    let whence = whence as usize;
    syscall3(SYS_LSEEK, fd, offset, whence).map(drop)
}

/// Set extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-lsetxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::lsetxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn lsetxattr<P: AsRef<Path>>(
    filename: P,
    name: P,
    value: usize,
    size: size_t,
    flags: i32,
) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(SYS_LSETXATTR, filename_ptr, name_ptr, value, size, flags).map(drop)
}

/// Get file status about a file, without following symbolic.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat_t::default();
/// let ret = unsafe { nc::lstat(path, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn lstat<P: AsRef<Path>>(filename: P, statbuf: &mut stat_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat_t as usize;
    syscall2(SYS_LSTAT, filename_ptr, statbuf_ptr).map(drop)
}

/// Get file status about a file, without following symbolic.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat64_t::default();
/// let ret = unsafe { nc::lstat64(path, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn lstat64<P: AsRef<Path>>(filename: P, statbuf: &mut stat64_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat64_t as usize;
    syscall2(SYS_LSTAT64, filename_ptr, statbuf_ptr).map(drop)
}

/// Give advice about use of memory.
///
/// # Example
///
/// ```
/// // Initialize an anonymous mapping with 4 pages.
/// let map_length = 4 * nc::PAGE_SIZE;
/// let addr = unsafe {
///     nc::mmap(
///         0,
///         map_length,
///         nc::PROT_READ | nc::PROT_WRITE,
///         nc::MAP_PRIVATE | nc::MAP_ANONYMOUS,
///         -1,
///         0,
///     )
/// };
/// assert!(addr.is_ok());
/// let addr = addr.unwrap();
///
/// // Set the third page readonly. And we will run into SIGSEGV when updating it.
/// let ret = unsafe { nc::madvise(addr + 2 * nc::PAGE_SIZE, nc::PAGE_SIZE, nc::MADV_RANDOM) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn madvise(addr: usize, len: size_t, advice: i32) -> Result<(), Errno> {
    let advice = advice as usize;
    syscall3(SYS_MADVISE, addr, len, advice).map(drop)
}

/// Set memory policy for a memory range.
pub unsafe fn mbind(
    start: usize,
    len: usize,
    mode: i32,
    nmask: *const usize,
    maxnode: usize,
    flags: i32,
) -> Result<(), Errno> {
    let mode = mode as usize;
    let nmask = nmask as usize;
    let flags = flags as usize;
    syscall6(SYS_MBIND, start, len, mode, nmask, maxnode, flags).map(drop)
}

/// Issue memory barriers on a set of threads.
///
/// @cmd:   Takes command values defined in enum `membarrier_cmd`.
/// @flags: Currently needs to be 0. For future extensions.
///
/// If this system call is not implemented, `-ENOSYS` is returned. If the
/// command specified does not exist, not available on the running
/// kernel, or if the command argument is invalid, this system call
/// returns `-EINVAL`. For a given command, with flags argument set to 0,
/// this system call is guaranteed to always return the same value until
/// reboot.
///
/// All memory accesses performed in program order from each targeted thread
/// is guaranteed to be ordered with respect to `sys_membarrier()`. If we use
/// the semantic `barrier()` to represent a compiler barrier forcing memory
/// accesses to be performed in program order across the barrier, and
/// `smp_mb()` to represent explicit memory barriers forcing full memory
/// ordering across the barrier, we have the following ordering table for
/// each pair of `barrier()`, `sys_membarrier()` and `smp_mb()`:
///
/// The pair ordering is detailed as (O: ordered, X: not ordered):
///
/// ```text
///                        barrier()   smp_mb() sys_membarrier()
///        barrier()          X           X            O
///        smp_mb()           X           O            O
///        sys_membarrier()   O           O            O
/// ```
pub unsafe fn membarrier(cmd: i32, flags: i32) -> Result<i32, Errno> {
    let cmd = cmd as usize;
    let flags = flags as usize;
    syscall2(SYS_MEMBARRIER, cmd, flags).map(|ret| ret as i32)
}

/// Create an anonymous file.
pub unsafe fn memfd_create<P: AsRef<Path>>(name: P, flags: u32) -> Result<i32, Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_MEMFD_CREATE, name_ptr, flags).map(|ret| ret as i32)
}

/// Move all pages in a process to another set of nodes
pub unsafe fn migrate_pages(
    pid: pid_t,
    maxnode: usize,
    old_nodes: *const usize,
    new_nodes: *const usize,
) -> Result<isize, Errno> {
    let pid = pid as usize;
    let old_nodes = old_nodes as usize;
    let new_nodes = new_nodes as usize;
    syscall4(SYS_MIGRATE_PAGES, pid, maxnode, old_nodes, new_nodes).map(|ret| ret as isize)
}

/// mincore() returns the memory residency status of the pages in the
/// current process's address space specified by [addr, addr + len).
/// The status is returned in a vector of bytes.  The least significant
/// bit of each byte is 1 if the referenced page is in memory, otherwise
/// it is zero.
///
/// Because the status of a page can change after mincore() checks it
/// but before it returns to the application, the returned vector may
/// contain stale information.  Only locked pages are guaranteed to
/// remain in memory.
///
/// return values:
///  zero    - success
///  -EFAULT - vec points to an illegal address
///  -EINVAL - addr is not a multiple of `PAGE_SIZE`
///  -ENOMEM - Addresses in the range [addr, addr + len] are
/// invalid for the address space of this process, or specify one or
/// more pages which are not currently mapped
///  -EAGAIN - A kernel resource was temporarily unavailable.
pub unsafe fn mincore(start: usize, len: size_t, vec: *const u8) -> Result<(), Errno> {
    let vec_ptr = vec as usize;
    syscall3(SYS_MINCORE, start, len, vec_ptr).map(drop)
}

/// Create a directory.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-mkdir";
/// let ret = unsafe { nc::mkdir(path, 0o755) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, nc::AT_REMOVEDIR) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mkdir<P: AsRef<Path>>(filename: P, mode: mode_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall2(SYS_MKDIR, filename_ptr, mode).map(drop)
}

/// Create a directory.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-mkdir";
/// let ret = unsafe { nc::mkdirat(nc::AT_FDCWD, path, 0o755) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, nc::AT_REMOVEDIR) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mkdirat<P: AsRef<Path>>(dirfd: i32, filename: P, mode: mode_t) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    syscall3(SYS_MKDIRAT, dirfd, filename_ptr, mode).map(drop)
}

/// Create a special or ordinary file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-mknod";
/// // Create a named pipe.
/// let ret = unsafe { nc::mknod(path, nc::S_IFIFO | nc::S_IRUSR | nc::S_IWUSR, 0) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mknod<P: AsRef<Path>>(filename: P, mode: mode_t, dev: dev_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    let dev = dev as usize;
    syscall3(SYS_MKNOD, filename_ptr, mode, dev).map(drop)
}

/// Create a special or ordinary file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-mknodat";
/// // Create a named pipe.
/// let ret = unsafe { nc::mknodat(nc::AT_FDCWD, path, nc::S_IFIFO | nc::S_IRUSR | nc::S_IWUSR, 0) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mknodat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    mode: mode_t,
    dev: dev_t,
) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let mode = mode as usize;
    let dev = dev as usize;
    syscall4(SYS_MKNODAT, dirfd, filename_ptr, mode, dev).map(drop)
}

/// Lock memory.
///
/// # Example
///
/// ```
/// let mut passwd_buf = [0_u8; 64];
/// let ret = unsafe { nc::mlock(passwd_buf.as_ptr() as usize, passwd_buf.len()) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mlock(addr: usize, len: size_t) -> Result<(), Errno> {
    syscall2(SYS_MLOCK, addr, len).map(drop)
}

/// Lock memory.
///
/// # Example
///
/// ```
/// let mut passwd_buf = [0_u8; 64];
/// let ret = unsafe { nc::mlock2(passwd_buf.as_ptr() as usize, passwd_buf.len(), nc::MCL_CURRENT) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mlock2(addr: usize, len: size_t, flags: i32) -> Result<(), Errno> {
    let flags = flags as usize;
    syscall3(SYS_MLOCK2, addr, len, flags).map(drop)
}

/// Lock memory.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::mlockall(nc::MCL_CURRENT) };
/// // We got out-of-memory error in CI environment.
/// assert!(ret.is_ok() || ret == Err(nc::ENOMEM));
/// ```
pub unsafe fn mlockall(flags: i32) -> Result<(), Errno> {
    let flags = flags as usize;
    syscall1(SYS_MLOCKALL, flags).map(drop)
}

/// Map files or devices into memory.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let mut sb = nc::stat_t::default();
/// let ret = unsafe { nc::fstat(fd, &mut sb) };
/// assert!(ret.is_ok());
///
/// let offset: usize = 0;
/// let length: usize = sb.st_size as usize - offset;
/// // Offset for mmap must be page aligned.
/// let pa_offset: usize = offset & !(nc::PAGE_SIZE - 1);
/// let map_length = length + offset - pa_offset;
///
/// let addr = unsafe {
///     nc::mmap(
///         0, // 0 as NULL
///         map_length,
///         nc::PROT_READ,
///         nc::MAP_PRIVATE,
///         fd,
///         pa_offset as nc::off_t,
///     )
/// };
/// assert!(addr.is_ok());
/// let addr = addr.unwrap();
///
/// let n_write = unsafe { nc::write(1, addr + offset - pa_offset, length) };
/// assert!(n_write.is_ok());
/// assert_eq!(n_write, Ok(length as nc::ssize_t));
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mmap(
    start: usize,
    len: size_t,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: off_t,
) -> Result<usize, Errno> {
    let prot = prot as usize;
    let flags = flags as usize;
    let fd = fd as usize;
    let offset = offset as usize;
    syscall6(SYS_MMAP, start, len, prot, flags, fd, offset)
}

/// Map files or devices into memory.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let mut sb = nc::stat_t::default();
/// let ret = unsafe { nc::fstat(fd, &mut sb) };
/// assert!(ret.is_ok());
///
/// let offset: usize = 0;
/// let length: usize = sb.st_size as usize - offset;
/// // Offset for mmap must be page aligned.
/// let pa_offset: usize = offset & !(nc::PAGE_SIZE - 1);
/// let map_length = length + offset - pa_offset;
///
/// let addr = unsafe {
///     nc::mmap2(
///         0, // 0 as NULL
///         map_length,
///         nc::PROT_READ,
///         nc::MAP_PRIVATE,
///         fd,
///         pa_offset as nc::off_t,
///     )
/// };
/// assert!(addr.is_ok());
/// let addr = addr.unwrap();
///
/// let n_write = unsafe { nc::write(1, addr + offset - pa_offset, length) };
/// assert!(n_write.is_ok());
/// assert_eq!(n_write, Ok(length as nc::ssize_t));
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mmap2(
    start: usize,
    len: size_t,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: off_t,
) -> Result<usize, Errno> {
    let prot = prot as usize;
    let flags = flags as usize;
    let fd = fd as usize;
    let offset = offset as usize;
    syscall6(SYS_MMAP2, start, len, prot, flags, fd, offset)
}

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

/// Move a mount from one place to another.
///
/// In combination with fsopen()/fsmount() this is used to install a new mount
/// and in combination with `open_tree(OPEN_TREE_CLONE [| AT_RECURSIVE])`
/// it can be used to copy a mount subtree.
///
/// Note the flags value is a combination of `MOVE_MOUNT_*` flags.
pub unsafe fn move_mount<P: AsRef<Path>>(
    from_dfd: i32,
    from_pathname: P,
    to_dfd: i32,
    to_pathname: P,
    flags: u32,
) -> Result<i32, Errno> {
    let from_dfd = from_dfd as usize;
    let from_pathname = PathBuf::new(from_pathname);
    let from_pathname_ptr = from_pathname.as_ptr() as usize;
    let to_dfd = to_dfd as usize;
    let to_pathname = PathBuf::new(to_pathname);
    let to_pathname_ptr = to_pathname.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(
        SYS_MOVE_MOUNT,
        from_dfd,
        from_pathname_ptr,
        to_dfd,
        to_pathname_ptr,
        flags,
    )
    .map(|ret| ret as i32)
}

/// Move individual pages of a process to another node
pub unsafe fn move_pages(
    pid: pid_t,
    nr_pages: usize,
    pages: usize,
    nodes: *const i32,
    status: &mut i32,
    flags: i32,
) -> Result<(), Errno> {
    let pid = pid as usize;
    let nodes_ptr = nodes as usize;
    let status = status as *mut i32 as usize;
    let flags = flags as usize;
    syscall6(
        SYS_MOVE_PAGES,
        pid,
        nr_pages,
        pages,
        nodes_ptr,
        status,
        flags,
    )
    .map(drop)
}

/// Set protection on a region of memory.
///
/// # Example
///
/// ```
/// // Initialize an anonymous mapping with 4 pages.
/// let map_length = 4 * nc::PAGE_SIZE;
/// let addr = unsafe {
///     nc::mmap(
///         0,
///         map_length,
///         nc::PROT_READ | nc::PROT_WRITE,
///         nc::MAP_PRIVATE | nc::MAP_ANONYMOUS,
///         -1,
///         0,
///     )
/// };
/// assert!(addr.is_ok());
/// let addr = addr.unwrap();
///
/// // Set the third page readonly. And we will run into SIGSEGV when updating it.
/// let ret = unsafe { nc::mprotect(addr + 2 * nc::PAGE_SIZE, nc::PAGE_SIZE, nc::PROT_READ) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mprotect(addr: usize, len: size_t, prot: i32) -> Result<(), Errno> {
    let prot = prot as usize;
    syscall3(SYS_MPROTECT, addr, len, prot).map(drop)
}

/// Get/set message queue attributes
///
/// # Example
///
/// ```
/// let name = "nc-mq-getsetattr";
/// let ret = unsafe {
///     nc::mq_open(
///         name,
///         nc::O_CREAT | nc::O_RDWR,
///         (nc::S_IRUSR | nc::S_IWUSR) as nc::umode_t,
///         None,
///     )
/// };
/// assert!(ret.is_ok());
/// let mq_id = ret.unwrap();
///
/// let mut attr = nc::mq_attr_t::default();
/// let ret = unsafe { nc::mq_getsetattr(mq_id, None, Some(&mut attr)) };
/// assert!(ret.is_ok());
/// println!("attr: {:?}", attr);
///
/// let ret = unsafe { nc::close(mq_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::mq_unlink(name) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mq_getsetattr(
    mqdes: mqd_t,
    new_attr: Option<&mut mq_attr_t>,
    old_attr: Option<&mut mq_attr_t>,
) -> Result<mqd_t, Errno> {
    let mqdes = mqdes as usize;
    let new_attr_ptr = new_attr.map_or(0, |new_attr| new_attr as *mut mq_attr_t as usize);
    let old_attr_ptr = old_attr.map_or(0, |old_attr| old_attr as *mut mq_attr_t as usize);
    syscall3(SYS_MQ_GETSETATTR, mqdes, new_attr_ptr, old_attr_ptr).map(|ret| ret as mqd_t)
}

/// Register for notification when a message is available
pub unsafe fn mq_notify(mqdes: mqd_t, notification: Option<&sigevent_t>) -> Result<(), Errno> {
    let mqdes = mqdes as usize;
    let notification_ptr =
        notification.map_or(0, |notification| notification as *const sigevent_t as usize);
    syscall2(SYS_MQ_NOTIFY, mqdes, notification_ptr).map(drop)
}

/// Open a message queue.
///
/// # Example
///
/// ```
/// let name = "nc-posix-mq";
/// let ret = unsafe {
///     nc::mq_open(
///         name,
///         nc::O_CREAT | nc::O_RDWR,
///         (nc::S_IRUSR | nc::S_IWUSR) as nc::umode_t,
///         None,
///     )
/// };
/// assert!(ret.is_ok());
/// let mq_id = ret.unwrap();
/// let ret = unsafe { nc::close(mq_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::mq_unlink(name) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mq_open<P: AsRef<Path>>(
    name: P,
    oflag: i32,
    mode: umode_t,
    attr: Option<&mut mq_attr_t>,
) -> Result<mqd_t, Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let oflag = oflag as usize;
    let mode = mode as usize;
    let attr_ptr = attr.map_or(0, |attr| attr as *mut mq_attr_t as usize);
    syscall4(SYS_MQ_OPEN, name_ptr, oflag, mode, attr_ptr).map(|ret| ret as mqd_t)
}

/// Receive a message from a message queue
///
/// # Example
///
/// ```
/// let name = "nc-mq-timedreceive";
/// let ret = unsafe {
///     nc::mq_open(
///         name,
///         nc::O_CREAT | nc::O_RDWR | nc::O_EXCL,
///         (nc::S_IRUSR | nc::S_IWUSR) as nc::umode_t,
///         None,
///     )
/// };
/// assert!(ret.is_ok());
/// let mq_id = ret.unwrap();
///
/// let mut attr = nc::mq_attr_t::default();
/// let ret = unsafe { nc::mq_getsetattr(mq_id, None, Some(&mut attr)) };
/// assert!(ret.is_ok());
/// println!("attr: {:?}", attr);
///
/// let msg = "Hello, Rust";
/// let prio = 42;
/// let timeout = nc::timespec_t {
///     tv_sec: 1,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::mq_timedsend(mq_id, msg.as_bytes(), msg.len(), prio, &timeout) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::mq_getsetattr(mq_id, None, Some(&mut attr)) };
/// assert!(ret.is_ok());
/// assert_eq!(attr.mq_curmsgs, 1);
///
/// let mut buf = vec![0_u8; attr.mq_msgsize as usize];
/// let buf_len = buf.len();
/// let mut recv_prio = 0;
/// let read_timeout = nc::timespec_t {
///     tv_sec: 1,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::mq_timedreceive(mq_id, &mut buf, buf_len, &mut recv_prio, &read_timeout) };
/// if let Err(errno) = ret {
///     eprintln!("mq_timedreceive() error: {}", nc::strerror(errno));
/// }
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap() as usize;
/// assert_eq!(n_read, msg.len());
///
/// let ret = unsafe { nc::close(mq_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::mq_unlink(name) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mq_timedreceive(
    mqdes: mqd_t,
    msg: &mut [u8],
    msg_len: usize,
    msg_prio: &mut u32,
    abs_timeout: &timespec_t,
) -> Result<ssize_t, Errno> {
    let mqdes = mqdes as usize;
    let msg = PathBuf::new(msg);
    let msg_ptr = msg.as_ptr() as usize;
    let msg_prio = msg_prio as *mut u32 as usize;
    let abs_timeout_ptr = abs_timeout as *const timespec_t as usize;
    syscall5(
        SYS_MQ_TIMEDRECEIVE,
        mqdes,
        msg_ptr,
        msg_len,
        msg_prio,
        abs_timeout_ptr,
    )
    .map(|ret| ret as ssize_t)
}

/// Send message to a message queue.
///
/// # Example
///
/// ```
/// let name = "nc-mq-timedsend";
/// let ret = unsafe {
///     nc::mq_open(
///         name,
///         nc::O_CREAT | nc::O_RDWR,
///         (nc::S_IRUSR | nc::S_IWUSR) as nc::umode_t,
///         None,
///     )
/// };
/// assert!(ret.is_ok());
/// let mq_id = ret.unwrap();
///
/// let mut attr = nc::mq_attr_t::default();
/// let ret = unsafe { nc::mq_getsetattr(mq_id, None, Some(&mut attr)) };
/// assert!(ret.is_ok());
///
/// let msg = "Hello, Rust";
/// let prio = 0;
/// let timeout = nc::timespec_t {
///     tv_sec: 1,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::mq_timedsend(mq_id, msg.as_bytes(), msg.len(), prio, &timeout) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::mq_getsetattr(mq_id, None, Some(&mut attr)) };
/// assert!(ret.is_ok());
/// assert_eq!(attr.mq_curmsgs, 1);
///
/// let ret = unsafe { nc::close(mq_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::mq_unlink(name) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mq_timedsend(
    mqdes: mqd_t,
    msg: &[u8],
    msg_len: usize,
    msg_prio: u32,
    abs_timeout: &timespec_t,
) -> Result<(), Errno> {
    let mqdes = mqdes as usize;
    let msg = PathBuf::new(msg);
    let msg_ptr = msg.as_ptr() as usize;
    let msg_prio = msg_prio as usize;
    let abs_timeout_ptr = abs_timeout as *const timespec_t as usize;
    syscall5(
        SYS_MQ_TIMEDSEND,
        mqdes,
        msg_ptr,
        msg_len,
        msg_prio,
        abs_timeout_ptr,
    )
    .map(drop)
}

/// Remove a message queue.
///
/// # Example
///
/// ```
/// let name = "nc-mq-unlink";
/// let ret = unsafe {
///     nc::mq_open(
///         name,
///         nc::O_CREAT | nc::O_RDWR,
///         (nc::S_IRUSR | nc::S_IWUSR) as nc::umode_t,
///         None,
///     )
/// };
/// assert!(ret.is_ok());
/// let mq_id = ret.unwrap();
/// let ret = unsafe { nc::close(mq_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::mq_unlink(name) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn mq_unlink<P: AsRef<Path>>(name: P) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall1(SYS_MQ_UNLINK, name_ptr).map(drop)
}

/// Remap a virtual memory address
pub unsafe fn mremap(
    addr: usize,
    old_len: size_t,
    new_len: size_t,
    flags: usize,
    new_addr: usize,
) -> Result<usize, Errno> {
    syscall5(SYS_MREMAP, addr, old_len, new_len, flags, new_addr)
}

/// System V message control operations.
///
/// # Example
///
/// ```
/// let key = nc::IPC_PRIVATE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | (nc::S_IRUSR | nc::S_IWUSR) as i32;
/// let ret = unsafe { nc::msgget(key, flags) };
/// assert!(ret.is_ok());
/// let msq_id = ret.unwrap();

/// let mut buf = nc::msqid_ds_t::default();
/// let ret = unsafe { nc::msgctl(msq_id, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn msgctl(msqid: i32, cmd: i32, buf: &mut msqid_ds_t) -> Result<i32, Errno> {
    let msqid = msqid as usize;
    let cmd = cmd as usize;
    let buf_ptr = buf as *mut msqid_ds_t as usize;
    syscall3(SYS_MSGCTL, msqid, cmd, buf_ptr).map(|ret| ret as i32)
}

/// Get a System V message queue identifier.
///
/// # Example
///
/// ```
/// let key = nc::IPC_PRIVATE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | (nc::S_IRUSR | nc::S_IWUSR) as i32;
/// let ret = unsafe { nc::msgget(key, flags) };
/// assert!(ret.is_ok());
/// let msq_id = ret.unwrap();

/// let mut buf = nc::msqid_ds_t::default();
/// let ret = unsafe { nc::msgctl(msq_id, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn msgget(key: key_t, msgflg: i32) -> Result<i32, Errno> {
    let key = key as usize;
    let msgflg = msgflg as usize;
    syscall2(SYS_MSGGET, key, msgflg).map(|ret| ret as i32)
}

/// Receive messages from a System V message queue.
///
/// # Example
///
/// ```
/// const MAX_MTEXT: usize = 1024;
///
/// const MTYPE_NULL: isize = 0;
/// const MTYPE_CLIENT: isize = 1;
/// const _MTYPE_SERVER: isize = 2;
///
/// #[derive(Debug, Clone, Copy)]
/// struct Message {
///     pub mtype: isize,
///     pub mtext: [u8; MAX_MTEXT],
/// }
///
/// impl Default for Message {
///     fn default() -> Self {
///         Message {
///             mtype: MTYPE_NULL,
///             mtext: [0; MAX_MTEXT],
///         }
///     }
/// }
///
/// fn main() {
///     let key = nc::IPC_PRIVATE;
///     let flags = nc::IPC_CREAT | nc::IPC_EXCL | (nc::S_IRUSR | nc::S_IWUSR) as i32;
///     let ret = unsafe { nc::msgget(key, flags) };
///     assert!(ret.is_ok());
///     let msq_id = ret.unwrap();
///
///     // Write to message queue.
///     let msg = "Hello, Rust";
///     let mut client_msg = Message {
///         mtype: MTYPE_CLIENT,
///         mtext: [0; MAX_MTEXT],
///     };
///     let msg_len = msg.len();
///     unsafe {
///         let src_ptr = msg.as_ptr();
///         let dst_ptr = client_msg.mtext.as_mut_ptr();
///         core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, msg_len);
///     }
///
///     let ret = unsafe { nc::msgsnd(msq_id, &client_msg as *const Message as usize, msg_len, 0) };
///     assert!(ret.is_ok());
///
///     // Read from message queue.
///     let mut recv_msg = Message::default();
///     let ret = unsafe {
///         nc::msgrcv(
///             msq_id,
///             &mut recv_msg as *mut Message as usize,
///             MAX_MTEXT,
///             MTYPE_CLIENT,
///             0,
///         )
///     };
///     assert!(ret.is_ok());
///     let recv_msg_len = ret.unwrap() as usize;
///     assert_eq!(recv_msg_len, msg_len);
///     let recv_text = core::str::from_utf8(&recv_msg.mtext[..recv_msg_len]);
///     assert!(recv_text.is_ok());
///     let recv_text = recv_text.unwrap();
///     assert_eq!(recv_text, msg);
///     println!("recv text: {}", recv_text);
///
///     let mut buf = nc::msqid_ds_t::default();
///     let ret = unsafe { nc::msgctl(msq_id, nc::IPC_RMID, &mut buf) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn msgrcv(
    msqid: i32,
    msgq: usize,
    msgsz: size_t,
    msgtyp: isize,
    msgflg: i32,
) -> Result<ssize_t, Errno> {
    let msqid = msqid as usize;
    let msgtyp = msgtyp as usize;
    let msgflg = msgflg as usize;
    syscall5(SYS_MSGRCV, msqid, msgq, msgsz, msgtyp, msgflg).map(|ret| ret as ssize_t)
}

/// Append the message to a System V message queue.
///
/// # Example
///
/// ```
/// const MAX_MTEXT: usize = 1024;
///
/// const MTYPE_NULL: isize = 0;
/// const MTYPE_CLIENT: isize = 1;
/// const _MTYPE_SERVER: isize = 2;
///
/// #[derive(Debug, Clone, Copy)]
/// struct Message {
///     pub mtype: isize,
///     pub mtext: [u8; MAX_MTEXT],
/// }
///
/// impl Default for Message {
///     fn default() -> Self {
///         Message {
///             mtype: MTYPE_NULL,
///             mtext: [0; MAX_MTEXT],
///         }
///     }
/// }
///
/// fn main() {
///     let key = nc::IPC_PRIVATE;
///     let flags = nc::IPC_CREAT | nc::IPC_EXCL | (nc::S_IRUSR | nc::S_IWUSR) as i32;
///     let ret = unsafe { nc::msgget(key, flags) };
///     assert!(ret.is_ok());
///     let msq_id = ret.unwrap();
///
///     // Write to message queue.
///     let msg = "Hello, Rust";
///     let mut client_msg = Message {
///         mtype: MTYPE_CLIENT,
///         mtext: [0; MAX_MTEXT],
///     };
///     let msg_len = msg.len();
///     unsafe {
///         let src_ptr = msg.as_ptr();
///         let dst_ptr = client_msg.mtext.as_mut_ptr();
///         core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, msg_len);
///     }
///
///     let ret = unsafe { nc::msgsnd(msq_id, &client_msg as *const Message as usize, msg_len, 0) };
///     assert!(ret.is_ok());
///
///     // Read from message queue.
///     let mut recv_msg = Message::default();
///     let ret = unsafe {
///         nc::msgrcv(
///             msq_id,
///             &mut recv_msg as *mut Message as usize,
///             MAX_MTEXT,
///             MTYPE_CLIENT,
///             0,
///         )
///     };
///     assert!(ret.is_ok());
///     let recv_msg_len = ret.unwrap() as usize;
///     assert_eq!(recv_msg_len, msg_len);
///     let recv_text = core::str::from_utf8(&recv_msg.mtext[..recv_msg_len]);
///     assert!(recv_text.is_ok());
///     let recv_text = recv_text.unwrap();
///     assert_eq!(recv_text, msg);
///
///     let mut buf = nc::msqid_ds_t::default();
///     let ret = unsafe { nc::msgctl(msq_id, nc::IPC_RMID, &mut buf) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn msgsnd(msqid: i32, msgq: usize, msgsz: size_t, msgflg: i32) -> Result<(), Errno> {
    let msqid = msqid as usize;
    let msgflg = msgflg as usize;
    syscall4(SYS_MSGSND, msqid, msgq, msgsz, msgflg).map(drop)
}

/// Synchronize a file with memory map.
pub unsafe fn msync(addr: usize, len: size_t, flags: i32) -> Result<(), Errno> {
    let flags = flags as usize;
    syscall3(SYS_MSYNC, addr, len, flags).map(drop)
}

/// Unlock memory.
///
/// # Example
///
/// ```
/// let mut passwd_buf = [0_u8; 64];
/// let addr = passwd_buf.as_ptr() as usize;
/// let ret = unsafe { nc::mlock2(addr, passwd_buf.len(), nc::MCL_CURRENT) };
/// for i in 0..passwd_buf.len() {
///   passwd_buf[i] = i as u8;
/// }
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::munlock(addr, passwd_buf.len()) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn munlock(addr: usize, len: size_t) -> Result<(), Errno> {
    syscall2(SYS_MUNLOCK, addr, len).map(drop)
}

/// Unlock memory.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::mlockall(nc::MCL_CURRENT) };
/// assert!(ret.is_ok() || ret == Err(nc::ENOMEM));
/// let ret = unsafe { nc::munlockall() };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn munlockall() -> Result<(), Errno> {
    syscall0(SYS_MUNLOCKALL).map(drop)
}

/// Unmap files or devices from memory.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let mut sb = nc::stat_t::default();
/// let ret = unsafe { nc::fstat(fd, &mut sb) };
/// assert!(ret.is_ok());
///
/// let offset: usize = 0;
/// let length: usize = sb.st_size as usize - offset;
/// // Offset for mmap must be page aligned.
/// let pa_offset: usize = offset & !(nc::PAGE_SIZE - 1);
/// let map_length = length + offset - pa_offset;
///
/// let addr = unsafe {
///     nc::mmap(
///         0, // 0 as NULL
///         map_length,
///         nc::PROT_READ,
///         nc::MAP_PRIVATE,
///         fd,
///         pa_offset as nc::off_t,
///     )
/// };
/// assert!(addr.is_ok());
/// let addr = addr.unwrap();
///
/// let n_write = unsafe { nc::write(1, addr + offset - pa_offset, length) };
/// assert!(n_write.is_ok());
/// assert_eq!(n_write, Ok(length as nc::ssize_t));
/// let ret = unsafe { nc::munmap(addr, map_length) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn munmap(addr: usize, len: size_t) -> Result<(), Errno> {
    syscall2(SYS_MUNMAP, addr, len).map(drop)
}

/// Obtain handle for a filename
pub unsafe fn name_to_handle_at<P: AsRef<Path>>(
    dfd: i32,
    filename: P,
    handle: &mut file_handle_t,
    mount_id: &mut i32,
    flags: i32,
) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let handle_ptr = handle as *mut file_handle_t as usize;
    let mount_id_ptr = mount_id as *mut i32 as usize;
    let flags = flags as usize;
    syscall5(
        SYS_NAME_TO_HANDLE_AT,
        dfd,
        filename_ptr,
        handle_ptr,
        mount_id_ptr,
        flags,
    )
    .map(drop)
}

/// High resolution sleep.
///
/// # Example
///
/// ```
/// let t = nc::timespec_t {
///     tv_sec: 1,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::nanosleep(&t, None) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn nanosleep(req: &timespec_t, rem: Option<&mut timespec_t>) -> Result<(), Errno> {
    let req_ptr = req as *const timespec_t as usize;
    let rem_ptr = rem.map_or(0, |rem| rem as *mut timespec_t as usize);
    syscall2(SYS_NANOSLEEP, req_ptr, rem_ptr).map(drop)
}

/// Change the priority of current process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::nice(5) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn nice(increment: i32) -> Result<(), Errno> {
    let increment = increment as usize;
    syscall1(SYS_NICE, increment).map(drop)
}

/// Open and possibly create a file.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::open(path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn open<P: AsRef<Path>>(filename: P, flags: i32, mode: mode_t) -> Result<i32, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flags = flags as usize;
    let mode = mode as usize;
    syscall3(SYS_OPEN, filename_ptr, flags, mode).map(|ret| ret as i32)
}

/// Open and possibly create a file within a directory.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn openat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    flags: i32,
    mode: mode_t,
) -> Result<i32, Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flags = flags as usize;
    let mode = mode as usize;
    syscall4(SYS_OPENAT, dirfd, filename_ptr, flags, mode).map(|ret| ret as i32)
}

/// Open and possibly create a file (extended)
///
/// # Example
///
/// ```
/// use core::mem::size_of;
/// let path = "/etc/passwd";
/// let mut how = nc::open_how_t{
///   flags: nc::O_RDONLY as u64,
///   ..nc::open_how_t::default()
/// };
/// let how_size = size_of::<nc::open_how_t>();
/// let ret = unsafe { nc::openat2(nc::AT_FDCWD, path, &mut how, how_size) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn openat2<P: AsRef<Path>>(
    dirfd: i32,
    pathname: P,
    how: *mut open_how_t,
    size: size_t,
) -> Result<i32, Errno> {
    let dirfd = dirfd as usize;
    let pathname = PathBuf::new(pathname);
    let pathname_ptr = pathname.as_ptr() as usize;
    let how_ptr = how as *const open_how_t as usize;
    syscall4(SYS_OPENAT2, dirfd, pathname_ptr, how_ptr, size).map(|ret| ret as i32)
}

/// Obtain handle for an open file
pub unsafe fn open_by_handle_at(
    mount_fd: i32,
    handle: &mut file_handle_t,
    flags: i32,
) -> Result<i32, Errno> {
    let mount_fd = mount_fd as usize;
    let handle_ptr = handle as *mut file_handle_t as usize;
    let flags = flags as usize;
    syscall3(SYS_OPEN_BY_HANDLE_AT, mount_fd, handle_ptr, flags).map(|ret| ret as i32)
}

pub unsafe fn open_tree<P: AsRef<Path>>(dfd: i32, filename: P, flags: u32) -> Result<i32, Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flags = flags as usize;
    syscall3(SYS_OPEN_TREE, dfd, filename_ptr, flags).map(|ret| ret as i32)
}

/// Pause the calling process to sleep until a signal is delivered.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
/// }
///
/// let sa = nc::sigaction_t {
///     sa_handler: handle_alarm as nc::sighandler_t,
///     ..nc::sigaction_t::default()
/// };
/// let mut old_sa = nc::sigaction_t::default();
/// let ret = unsafe { nc::rt_sigaction(nc::SIGALRM, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
/// assert!(ret.is_ok());
/// let remaining = unsafe { nc::alarm(1) };
/// let ret = unsafe { nc::pause() };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EINTR));
/// assert_eq!(remaining, 0);
/// ```
pub unsafe fn pause() -> Result<(), Errno> {
    syscall0(SYS_PAUSE).map(drop)
}

/// Set up performance monitoring.
pub unsafe fn perf_event_open(
    attr: &mut perf_event_attr_t,
    pid: pid_t,
    cpu: i32,
    group_fd: i32,
    flags: usize,
) -> Result<i32, Errno> {
    let attr_ptr = attr as *mut perf_event_attr_t as usize;
    let pid = pid as usize;
    let cpu = cpu as usize;
    let group_fd = group_fd as usize;
    syscall5(SYS_PERF_EVENT_OPEN, attr_ptr, pid, cpu, group_fd, flags).map(|ret| ret as i32)
}

/// Set the process execution domain.
pub unsafe fn personality(persona: u32) -> Result<u32, Errno> {
    let persona = persona as usize;
    syscall1(SYS_PERSONALITY, persona).map(|ret| ret as u32)
}

/// Obtain a duplicate of another process's file descriptor.
///
/// # Example
///
/// ```
/// let pid = unsafe { nc::fork() };
/// const STDOUT_FD: i32 = 1;
/// assert!(pid.is_ok());
/// if pid == Ok(0) {
///     println!("In child process, pid: {}", unsafe { nc::getpid() });
///     let path = "/tmp/nc-pidfdopen";
///     let fd = unsafe {
///         nc::openat(
///             nc::AT_FDCWD,
///             path,
///             nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///             0o644,
///         )
///     };
///     assert!(fd.is_ok());
///     let fd = fd.unwrap();
///     let ret = unsafe { nc::dup3(fd, STDOUT_FD, 0) };
///     assert!(ret.is_ok());
///     println!("[child] stdout redirected to file!");
///
///     let t = nc::timespec_t {
///         tv_sec: 2,
///         tv_nsec: 0,
///     };
///     unsafe {
///         let ret = nc::nanosleep(&t, None);
///         assert!(ret.is_ok());
///         let ret = nc::close(fd);
///         assert!(ret.is_ok());
///         let ret = nc::unlinkat(nc::AT_FDCWD, path, 0);
///         assert!(ret.is_ok());
///         nc::exit(0);
///     }
/// }
///
/// let pid = pid.unwrap();
/// println!("[parent] child pid: {}", pid);
///
/// let t = nc::timespec_t {
///     tv_sec: 2,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::nanosleep(&t, None) };
/// assert!(ret.is_ok());
///
/// let pidfd = unsafe { nc::pidfd_open(pid, 0) };
/// if pidfd == Err(nc::errno::ENOSYS) {
///     eprintln!("PIDFD_OPEN syscall not supported in this system");
///     return;
/// }
/// let pidfd = pidfd.unwrap();
///
/// let child_stdout_fd = unsafe { nc::pidfd_getfd(pidfd, STDOUT_FD, 0) };
/// if child_stdout_fd == Err(nc::errno::ENOSYS) {
///     eprintln!("PIDFD_OPEN syscall not supported in this system");
///     return;
/// }
/// let child_stdout_fd = child_stdout_fd.unwrap();
/// let msg = "Hello, msg from parent process\n";
/// let ret = unsafe { nc::write(child_stdout_fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// let nwrite = ret.unwrap();
/// assert_eq!(nwrite as usize, msg.len());
///
/// let ret = unsafe { nc::close(pidfd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(child_stdout_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pidfd_getfd(pidfd: i32, target_fd: i32, flags: u32) -> Result<i32, Errno> {
    let pidfd = pidfd as usize;
    let target_fd = target_fd as usize;
    let flags = flags as usize;
    syscall3(SYS_PIDFD_GETFD, pidfd, target_fd, flags).map(|ret| ret as i32)
}

/// Obtain a file descriptor that refers to a process.
///
/// # Example
///
/// ```
/// let pid = unsafe { nc::fork() };
/// const STDOUT_FD: i32 = 1;
/// assert!(pid.is_ok());
/// if pid == Ok(0) {
///     println!("In child process, pid: {}", unsafe { nc::getpid() });
///     let path = "/tmp/nc-pidfdopen";
///     let fd = unsafe {
///         nc::openat(
///             nc::AT_FDCWD,
///             path,
///             nc::O_CREAT | nc::O_WRONLY | nc::O_TRUNC,
///             0o644,
///         )
///     };
///     assert!(fd.is_ok());
///     let fd = fd.unwrap();
///     let ret = unsafe { nc::dup3(fd, STDOUT_FD, 0) };
///     assert!(ret.is_ok());
///
///     let t = nc::timespec_t {
///         tv_sec: 2,
///         tv_nsec: 0,
///     };
///     let ret = unsafe { nc::nanosleep(&t, None) };
///     assert!(ret.is_ok());
///     let ret = unsafe { nc::close(fd) };
///     assert!(ret.is_ok());
///     let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
///     assert!(ret.is_ok());
///     unsafe { nc::exit(0) };
/// }
///
/// let pid = pid.unwrap();
///
/// let t = nc::timespec_t {
///     tv_sec: 2,
///     tv_nsec: 0,
/// };
/// let ret = unsafe { nc::nanosleep(&t, None) };
/// assert!(ret.is_ok());
///
/// let pidfd = unsafe { nc::pidfd_open(pid, 0) };
/// if pidfd == Err(nc::errno::ENOSYS) {
///     eprintln!("PIDFD_OPEN syscall not supported in this system");
///     return;
/// }
/// let pidfd = pidfd.unwrap();
///
/// let child_stdout_fd = unsafe { nc::pidfd_getfd(pidfd, STDOUT_FD, 0) };
/// if child_stdout_fd == Err(nc::errno::ENOSYS) {
///     eprintln!("PIDFD_OPEN syscall not supported in this system");
///     return;
/// }
/// let child_stdout_fd = child_stdout_fd.unwrap();
/// let msg = "Hello, msg from parent process\n";
/// let ret = unsafe { nc::write(child_stdout_fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// let nwrite = ret.unwrap();
/// assert_eq!(nwrite as usize, msg.len());
///
/// let ret = unsafe { nc::close(pidfd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(child_stdout_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pidfd_open(pid: pid_t, flags: u32) -> Result<i32, Errno> {
    let pid = pid as usize;
    let flags = flags as usize;
    syscall2(SYS_PIDFD_OPEN, pid, flags).map(|ret| ret as i32)
}

/// Signal a process through a pidfd.
///
/// @pidfd:  file descriptor of the process
/// @sig:    signal to send
/// @info:   signal info
/// @flags:  future flags
///
/// The syscall currently only signals via `PIDTYPE_PID` which covers
/// kill(<positive-pid>, <signal>. It does not signal threads or process
/// groups.
/// In order to extend the syscall to threads and process groups the @flags
/// argument should be used. In essence, the @flags argument will determine
/// what is signaled and not the file descriptor itself. Put in other words,
/// grouping is a property of the flags argument not a property of the file
/// descriptor.
///
/// Return: 0 on success, negative errno on failure
pub unsafe fn pidfd_send_signal(
    pidfd: i32,
    sig: i32,
    info: &mut siginfo_t,
    flags: u32,
) -> Result<(), Errno> {
    let pidfd = pidfd as usize;
    let sig = sig as usize;
    let info_ptr = info as *mut siginfo_t as usize;
    let flags = flags as usize;
    syscall4(SYS_PIDFD_SEND_SIGNAL, pidfd, sig, info_ptr, flags).map(drop)
}

/// Create a pipe.
///
/// # Example
///
/// ```
/// let mut fds = [-1_i32, 2];
/// let ret = unsafe { nc::pipe(&mut fds) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[0]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[1]) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pipe(pipefd: &mut [i32; 2]) -> Result<(), Errno> {
    let pipefd_ptr = pipefd.as_mut_ptr() as usize;
    syscall1(SYS_PIPE, pipefd_ptr).map(drop)
}

/// Create a pipe.
///
/// # Example
///
/// ```
/// let mut fds = [-1_i32, 2];
/// let ret = unsafe {nc::pipe2(&mut fds, nc::O_CLOEXEC | nc::O_NONBLOCK) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[0]) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fds[1]) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pipe2(pipefd: &mut [i32; 2], flags: i32) -> Result<(), Errno> {
    let pipefd_ptr = pipefd.as_mut_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_PIPE2, pipefd_ptr, flags).map(drop)
}

/// Change the root filesystem.
pub unsafe fn pivot_root<P: AsRef<Path>>(new_root: P, put_old: P) -> Result<(), Errno> {
    let new_root = PathBuf::new(new_root);
    let new_root_ptr = new_root.as_ptr() as usize;
    let put_old = PathBuf::new(put_old);
    let put_old_ptr = put_old.as_ptr() as usize;
    syscall2(SYS_PIVOT_ROOT, new_root_ptr, put_old_ptr).map(drop)
}

/// Create a new protection key.
pub unsafe fn pkey_alloc(flags: usize, init_val: usize) -> Result<i32, Errno> {
    syscall2(SYS_PKEY_ALLOC, flags, init_val).map(|ret| ret as i32)
}

/// Free a protection key.
pub unsafe fn pkey_free(pkey: i32) -> Result<(), Errno> {
    let pkey = pkey as usize;
    syscall1(SYS_PKEY_FREE, pkey).map(drop)
}

/// Set protection on a region of memory.
pub unsafe fn pkey_mprotect(
    start: usize,
    len: size_t,
    prot: usize,
    pkey: i32,
) -> Result<(), Errno> {
    let pkey = pkey as usize;
    syscall4(SYS_PKEY_MPROTECT, start, len, prot, pkey).map(drop)
}

/// Wait for some event on file descriptors.
pub unsafe fn poll(fds: &mut [pollfd_t], timeout: i32) -> Result<(), Errno> {
    let fds_ptr = fds.as_mut_ptr() as usize;
    let nfds = fds.len();
    let timeout = timeout as usize;
    syscall3(SYS_POLL, fds_ptr, nfds, timeout).map(drop)
}

/// Wait for some event on a file descriptor.
pub unsafe fn ppoll(
    fds: &mut pollfd_t,
    nfds: i32,
    timeout: &timespec_t,
    sigmask: &sigset_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    let fds_ptr = fds as *mut pollfd_t as usize;
    let nfds = nfds as usize;
    let timeout_ptr = timeout as *const timespec_t as usize;
    let sigmask_ptr = sigmask as *const sigset_t as usize;
    syscall5(
        SYS_PPOLL,
        fds_ptr,
        nfds,
        timeout_ptr,
        sigmask_ptr,
        sigsetsize,
    )
    .map(|ret| ret as i32)
}

/// Operations on a process.
pub unsafe fn prctl(
    option: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<i32, Errno> {
    let option = option as usize;
    syscall5(SYS_PRCTL, option, arg2, arg3, arg4, arg5).map(|ret| ret as i32)
}

/// Read from a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [0_u8; 128];
/// let read_count = 64;
/// let ret = unsafe { nc::pread64(fd, buf.as_mut_ptr() as usize, read_count, 0) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(read_count as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pread64(fd: i32, buf: usize, count: usize, offset: off_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    syscall4(SYS_PREAD64, fd, buf, count, offset).map(|ret| ret as ssize_t)
}

/// Read from a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let iov_len = iov.len();
/// let ret = unsafe { nc::preadv(fd, &mut iov, 0, iov_len - 1) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn preadv(
    fd: i32,
    vec: &mut [iovec_t],
    pos_l: usize,
    pos_h: usize,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let vec_ptr = vec.as_mut_ptr() as usize;
    let vec_len = vec.len();
    syscall5(SYS_PREADV, fd, vec_ptr, vec_len, pos_l, pos_h).map(|ret| ret as ssize_t)
}

/// Read from a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let iov_len = iov.len();
/// let flags = 0;
/// let ret = unsafe { nc::preadv2(fd, &mut iov, 0, iov_len - 1, flags) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn preadv2(
    fd: i32,
    vec: &mut [iovec_t],
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let vec_ptr = vec.as_mut_ptr() as usize;
    let vec_len = vec.len();
    let flags = flags as usize;
    syscall6(SYS_PREADV2, fd, vec_ptr, vec_len, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
}

/// Get/set the resource limits of an arbitary process.
///
/// # Example
///
/// ```
/// let mut old_limit = nc::rlimit64_t::default();
/// let ret = unsafe { nc::prlimit64(nc::getpid(), nc::RLIMIT_NOFILE, None, Some(&mut old_limit)) };
/// assert!(ret.is_ok());
/// assert!(old_limit.rlim_cur > 0);
/// assert!(old_limit.rlim_max > 0);
/// ```
pub unsafe fn prlimit64(
    pid: pid_t,
    resource: i32,
    new_limit: Option<&rlimit64_t>,
    old_limit: Option<&mut rlimit64_t>,
) -> Result<(), Errno> {
    let pid = pid as usize;
    let resource = resource as usize;
    let new_limit_ptr = new_limit.map_or(0, |new_limit| new_limit as *const rlimit64_t as usize);
    let old_limit_ptr = old_limit.map_or(0, |old_limit| old_limit as *mut rlimit64_t as usize);
    syscall4(SYS_PRLIMIT64, pid, resource, new_limit_ptr, old_limit_ptr).map(drop)
}

/// Transfer data between process address spaces
pub unsafe fn process_vm_readv(
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
        SYS_PROCESS_VM_READV,
        pid,
        lvec_ptr,
        lvec_len,
        rvec_ptr,
        rvec_len,
        flags,
    )
    .map(|ret| ret as ssize_t)
}

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

/// Sychronous I/O multiplexing.
///
/// Most architectures can't handle 7-argument syscalls. So we provide a
/// 6-argument version where the sixth argument is a pointer to a structure
/// which has a pointer to the `sigset_t` itself followed by a `size_t` containing
/// the sigset size.
pub unsafe fn pselect6(
    nfds: i32,
    readfds: &mut fd_set_t,
    writefds: &mut fd_set_t,
    exceptfds: &mut fd_set_t,
    timeout: &timespec_t,
    sigmask: &sigset_t,
) -> Result<i32, Errno> {
    let nfds = nfds as usize;
    let readfds_ptr = readfds as *mut fd_set_t as usize;
    let writefds_ptr = writefds as *mut fd_set_t as usize;
    let exceptfds_ptr = exceptfds as *mut fd_set_t as usize;
    let timeout_ptr = timeout as *const timespec_t as usize;
    let sigmask_ptr = sigmask as *const sigset_t as usize;
    syscall6(
        SYS_PSELECT6,
        nfds,
        readfds_ptr,
        writefds_ptr,
        exceptfds_ptr,
        timeout_ptr,
        sigmask_ptr,
    )
    .map(|ret| ret as i32)
}

/// Process trace.
pub unsafe fn ptrace(request: i32, pid: pid_t, addr: usize, data: usize) -> Result<isize, Errno> {
    let request = request as usize;
    let pid = pid as usize;
    syscall4(SYS_PTRACE, request, pid, addr, data).map(|ret| ret as isize)
}

/// Write to a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-pwrite64";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let buf = "Hello, Rust";
/// let ret = unsafe { nc::pwrite64(fd, buf.as_ptr() as usize, buf.len(), 0) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(buf.len() as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pwrite64(
    fd: i32,
    buf: usize,
    count: size_t,
    offset: off_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    syscall4(SYS_PWRITE64, fd, buf, count, offset).map(|ret| ret as ssize_t)
}

/// Write to a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let ret = unsafe { nc::readv(fd, &mut iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
///
/// let path_out = "/tmp/nc-pwritev";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path_out, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::pwritev(fd, &iov, 0, iov.len() - 1) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path_out, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pwritev(
    fd: i32,
    vec: &[iovec_t],
    pos_l: usize,
    pos_h: usize,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let vec_ptr = vec.as_ptr() as usize;
    let vec_len = vec.len();
    syscall5(SYS_PWRITEV, fd, vec_ptr, vec_len, pos_l, pos_h).map(|ret| ret as ssize_t)
}

/// Write to a file descriptor without changing file offset.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let ret = unsafe { nc::readv(fd, &mut iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
///
/// let path_out = "/tmp/nc-pwritev2";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path_out, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let flags = nc::RWF_DSYNC | nc::RWF_APPEND;
/// let ret = unsafe { nc::pwritev2(fd, &iov, 0, iov.len() - 1, flags) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path_out, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pwritev2(
    fd: i32,
    vec: &[iovec_t],
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let vec_ptr = vec.as_ptr() as usize;
    let vec_len = vec.len();
    let flags = flags as usize;
    syscall6(SYS_PWRITEV2, fd, vec_ptr, vec_len, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
}

/// Manipulate disk quotes.
pub unsafe fn quotactl<P: AsRef<Path>>(
    cmd: i32,
    special: P,
    id: qid_t,
    addr: usize,
) -> Result<(), Errno> {
    let cmd = cmd as usize;
    let special = PathBuf::new(special);
    let special_ptr = special.as_ptr() as usize;
    let id = id as usize;
    syscall4(SYS_QUOTACTL, cmd, special_ptr, id, addr).map(drop)
}

/// Read from a file descriptor.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [0_u8; 4 * 1024];
/// let ret = unsafe { nc::read(fd, buf.as_mut_ptr() as usize, buf.len()) };
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap();
/// assert!(n_read <= buf.len() as nc::ssize_t);
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn read(fd: i32, buf_ptr: usize, count: size_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    syscall3(SYS_READ, fd, buf_ptr, count).map(|ret| ret as ssize_t)
}

/// Initialize file head into page cache.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::readahead(fd, 0, 64) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn readahead(fd: i32, offset: off_t, count: size_t) -> Result<(), Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    syscall3(SYS_READAHEAD, fd, offset, count).map(drop)
}

/// Read value of a symbolic link.
///
/// # Example
///
/// ```
/// let oldname = "/etc/passwd";
/// let newname = "/tmp/nc-readlink";
/// let ret = unsafe { nc::symlinkat(oldname, nc::AT_FDCWD, newname) };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; nc::PATH_MAX as usize];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::readlink(newname, &mut buf, buf_len) };
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap() as usize;
/// assert_eq!(n_read, oldname.len());
/// assert_eq!(oldname.as_bytes(), &buf[0..n_read]);
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, newname, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn readlink<P: AsRef<Path>>(
    filename: P,
    buf: &mut [u8],
    buf_len: size_t,
) -> Result<ssize_t, Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let buf_ptr = buf.as_mut_ptr() as usize;
    syscall3(SYS_READLINK, filename_ptr, buf_ptr, buf_len).map(|ret| ret as ssize_t)
}

/// Read value of a symbolic link.
///
/// # Example
///
/// ```
/// let oldname = "/etc/passwd";
/// let newname = "/tmp/nc-readlinkat";
/// let ret = unsafe { nc::symlinkat(oldname, nc::AT_FDCWD, newname) };
/// assert!(ret.is_ok());
/// let mut buf = [0_u8; nc::PATH_MAX as usize];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::readlinkat(nc::AT_FDCWD, newname, &mut buf, buf_len) };
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap() as usize;
/// assert_eq!(n_read, oldname.len());
/// assert_eq!(oldname.as_bytes(), &buf[0..n_read]);
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, newname, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn readlinkat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    buf: &mut [u8],
    buf_len: size_t,
) -> Result<ssize_t, Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let buf_ptr = buf.as_mut_ptr() as usize;
    syscall4(SYS_READLINKAT, dirfd, filename_ptr, buf_ptr, buf_len).map(|ret| ret as ssize_t)
}

/// Read from a file descriptor into multiple buffers.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
/// // TODO(Shaohua): Replace with as_mut_ptr()
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let ret = unsafe { nc::readv(fd, &mut iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn readv(fd: i32, iov: &mut [iovec_t]) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let iov_ptr = iov.as_mut_ptr() as usize;
    let len = iov.len();
    syscall3(SYS_READV, fd, iov_ptr, len).map(|ret| ret as ssize_t)
}

/// Reboot or enable/disable Ctrl-Alt-Del.
///
/// # Example
///
/// ```
/// let ret = unsafe {
///     nc::reboot(
///         nc::LINUX_REBOOT_MAGIC1,
///         nc::LINUX_REBOOT_MAGIC2,
///         nc::LINUX_REBOOT_CMD_RESTART,
///         0
///     )
/// };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn reboot(magic: i32, magci2: i32, cmd: u32, arg: usize) -> Result<(), Errno> {
    let magic = magic as usize;
    let magic2 = magci2 as usize;
    let cmd = cmd as usize;
    syscall4(SYS_REBOOT, magic, magic2, cmd, arg).map(drop)
}

/// Receive a message from a socket.
pub unsafe fn recvfrom(
    sockfd: i32,
    buf: &mut [u8],
    flags: i32,
    src_addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<ssize_t, Errno> {
    let sockfd = sockfd as usize;
    let buf_ptr = buf.as_mut_ptr() as usize;
    let buflen = buf.len();
    let flags = flags as usize;
    let src_addr_ptr = src_addr as *mut sockaddr_in_t as usize;
    let addrlen_ptr = addrlen as *mut socklen_t as usize;
    syscall6(
        SYS_RECVFROM,
        sockfd,
        buf_ptr,
        buflen,
        flags,
        src_addr_ptr,
        addrlen_ptr,
    )
    .map(|ret| ret as ssize_t)
}

/// Receives multile messages on a socket
pub unsafe fn recvmmsg(
    sockfd: i32,
    msgvec: &mut [mmsghdr_t],
    flags: i32,
    timeout: &mut timespec_t,
) -> Result<i32, Errno> {
    let sockfd = sockfd as usize;
    let msgvec_ptr = (msgvec as *mut [mmsghdr_t]).cast::<*mut mmsghdr_t>() as usize;
    let vlen = msgvec.len();
    let flags = flags as usize;
    let timeout_ptr = timeout as *mut timespec_t as usize;
    syscall5(SYS_RECVMMSG, sockfd, msgvec_ptr, vlen, flags, timeout_ptr).map(|ret| ret as i32)
}

/// Receive a msg from a socket.
pub unsafe fn recvmsg(sockfd: i32, msg: &mut msghdr_t, flags: i32) -> Result<ssize_t, Errno> {
    let sockfd = sockfd as usize;
    let msg_ptr = msg as *mut msghdr_t as usize;
    let flags = flags as usize;
    syscall3(SYS_RECVMSG, sockfd, msg_ptr, flags).map(|ret| ret as ssize_t)
}

/// Create a nonlinear file mapping.
/// Deprecated.
pub unsafe fn remap_file_pages(
    start: usize,
    size: size_t,
    prot: i32,
    pgoff: off_t,
    flags: i32,
) -> Result<(), Errno> {
    let prot = prot as usize;
    let pgoff = pgoff as usize;
    let flags = flags as usize;
    syscall5(SYS_REMAP_FILE_PAGES, start, size, prot, pgoff, flags).map(drop)
}

/// Remove an extended attribute.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-removexattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::removexattr(path, attr_name) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn removexattr<P: AsRef<Path>>(filename: P, name: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall2(SYS_REMOVEXATTR, filename_ptr, name_ptr).map(drop)
}

/// Change name or location of a file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-rename";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_path = "/tmp/nc-rename-new";
/// let ret = unsafe { nc::rename(path, new_path) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn rename<P: AsRef<Path>>(oldfilename: P, newfilename: P) -> Result<(), Errno> {
    let oldfilename = PathBuf::new(oldfilename);
    let oldfilename_ptr = oldfilename.as_ptr() as usize;
    let newfilename = PathBuf::new(newfilename);
    let newfilename_ptr = newfilename.as_ptr() as usize;
    syscall2(SYS_RENAME, oldfilename_ptr, newfilename_ptr).map(drop)
}

/// Change name or location of a file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-renameat";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_path = "/tmp/nc-renameat-new";
/// let ret = unsafe { nc::renameat(nc::AT_FDCWD, path, nc::AT_FDCWD, new_path) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn renameat<P: AsRef<Path>>(
    olddfd: i32,
    oldfilename: P,
    newdfd: i32,
    newfilename: P,
) -> Result<(), Errno> {
    let olddfd = olddfd as usize;
    let oldfilename = PathBuf::new(oldfilename);
    let oldfilename_ptr = oldfilename.as_ptr() as usize;
    let newdfd = newdfd as usize;
    let newfilename = PathBuf::new(newfilename);
    let newfilename_ptr = newfilename.as_ptr() as usize;
    syscall4(
        SYS_RENAMEAT,
        olddfd,
        oldfilename_ptr,
        newdfd,
        newfilename_ptr,
    )
    .map(drop)
}

/// Change name or location of a file.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-renameat2";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let new_path = "/tmp/nc-renameat2-new";
/// let flags = nc::RENAME_NOREPLACE;
/// let ret = unsafe { nc::renameat2(nc::AT_FDCWD, path, nc::AT_FDCWD, new_path, flags) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, new_path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn renameat2<P: AsRef<Path>>(
    olddfd: i32,
    oldfilename: P,
    newdfd: i32,
    newfilename: P,
    flags: i32,
) -> Result<(), Errno> {
    let olddfd = olddfd as usize;
    let oldfilename = PathBuf::new(oldfilename);
    let oldfilename_ptr = oldfilename.as_ptr() as usize;
    let newdfd = newdfd as usize;
    let newfilename = PathBuf::new(newfilename);
    let newfilename_ptr = newfilename.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(
        SYS_RENAMEAT2,
        olddfd,
        oldfilename_ptr,
        newdfd,
        newfilename_ptr,
        flags,
    )
    .map(drop)
}

/// Request a key from kernel's key management facility.
pub unsafe fn request_key<P: AsRef<Path>>(
    type_: P,
    description: P,
    callout_info: P,
    dest_keyring: key_serial_t,
) -> Result<key_serial_t, Errno> {
    let type_ = PathBuf::new(type_);
    let type_ptr = type_.as_ptr() as usize;
    let description = PathBuf::new(description);
    let description_ptr = description.as_ptr() as usize;
    let callout_info = PathBuf::new(callout_info);
    let callout_info_ptr = callout_info.as_ptr() as usize;
    let dest_keyring = dest_keyring as usize;
    syscall4(
        SYS_REQUEST_KEY,
        type_ptr,
        description_ptr,
        callout_info_ptr,
        dest_keyring,
    )
    .map(|ret| ret as key_serial_t)
}

/// Restart a system call after interruption by a stop signal.
pub unsafe fn restart_syscall() -> Result<i32, Errno> {
    syscall0(SYS_RESTART_SYSCALL).map(|ret| ret as i32)
}

/// Delete a directory.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-rmdir";
/// let ret = unsafe { nc::mkdirat(nc::AT_FDCWD, path, 0o755) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::rmdir(path) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn rmdir<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_RMDIR, filename_ptr).map(drop)
}

/// Setup restartable sequences for caller thread.
pub unsafe fn rseq(rseq: &mut [rseq_t], flags: i32, sig: u32) -> Result<i32, Errno> {
    let rseq_ptr = rseq.as_mut_ptr() as usize;
    let rseq_len = rseq.len();
    let flags = flags as usize;
    let sig = sig as usize;
    syscall4(SYS_RSEQ, rseq_ptr, rseq_len, flags, sig).map(|ret| ret as i32)
}

/// Examine and change a signal action.
///
/// # example
///
/// ```
/// use std::mem::size_of;
///
/// fn handle_sigterm(sig: i32) {
///     assert_eq!(sig, nc::SIGTERM);
/// }
///
/// let sa = nc::sigaction_t {
///     sa_handler: handle_sigterm as nc::sighandler_t,
///     sa_mask: nc::SA_RESTART | nc::SA_SIGINFO | nc::SA_ONSTACK,
///     ..nc::sigaction_t::default()
/// };
/// let mut old_sa = nc::sigaction_t::default();
/// let ret = unsafe { nc::rt_sigaction(nc::SIGTERM, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
/// let ret = unsafe { nc::kill(nc::getpid(), nc::SIGTERM) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn rt_sigaction(
    sig: i32,
    act: &sigaction_t,
    old_act: &mut sigaction_t,
    sigsetsize: size_t,
) -> Result<(), Errno> {
    let sig = sig as usize;
    let act_ptr = act as *const sigaction_t as usize;
    let old_act_ptr = old_act as *mut sigaction_t as usize;
    syscall4(SYS_RT_SIGACTION, sig, act_ptr, old_act_ptr, sigsetsize).map(drop)
}

/// Examine pending signals.
pub unsafe fn rt_sigpending(set: &mut [sigset_t]) -> Result<(), Errno> {
    let set_ptr = set.as_mut_ptr() as usize;
    syscall1(SYS_RT_SIGPENDING, set_ptr).map(drop)
}

/// Change the list of currently blocked signals.
pub unsafe fn rt_sigprocmask(
    how: i32,
    set: &sigset_t,
    oldset: &mut sigset_t,
    sigsetsize: size_t,
) -> Result<(), Errno> {
    let how = how as usize;
    let set_ptr = set as *const sigset_t as usize;
    let oldset_ptr = oldset as *mut sigset_t as usize;
    syscall4(SYS_RT_SIGPROCMASK, how, set_ptr, oldset_ptr, sigsetsize).map(drop)
}

/// Queue a signal and data.
pub unsafe fn rt_sigqueueinfo(pid: pid_t, sig: i32, uinfo: &mut siginfo_t) -> Result<(), Errno> {
    let pid = pid as usize;
    let sig = sig as usize;
    let uinfo_ptr = uinfo as *mut siginfo_t as usize;
    syscall3(SYS_RT_SIGQUEUEINFO, pid, sig, uinfo_ptr).map(drop)
}

/// Return from signal handler and cleanup stack frame.
///
/// Never returns.
pub unsafe fn rt_sigreturn() {
    let _ = syscall0(SYS_RT_SIGRETURN);
}

/// Wait for a signal.
///
/// Always returns Errno, normally EINTR.
///
/// # Example
/// ```
/// use core::mem::size_of;
///
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
///
/// if pid == 0 {
///     // child process.
///     let mask = nc::sigset_t::default();
///     let ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
/// } else {
///     // parent process.
///     let t = nc::timespec_t {
///         tv_sec: 1,
///         tv_nsec: 0,
///     };
///     let ret = unsafe { nc::nanosleep(&t, None) };
///     assert!(ret.is_ok());
///
///     let ret = unsafe { nc::kill(pid, nc::SIGTERM) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn rt_sigsuspend(set: &sigset_t, sigsetsize: size_t) -> Result<(), Errno> {
    let set_ptr = set as *const sigset_t as usize;
    syscall2(SYS_RT_SIGSUSPEND, set_ptr, sigsetsize).map(drop)
}

/// Synchronously wait for queued signals.
pub unsafe fn rt_sigtimedwait(
    uthese: &sigset_t,
    uinfo: &mut siginfo_t,
    uts: &timespec_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    let uthese_ptr = uthese as *const sigset_t as usize;
    let uinfo_ptr = uinfo as *mut siginfo_t as usize;
    let uts_ptr = uts as *const timespec_t as usize;
    syscall4(
        SYS_RT_SIGTIMEDWAIT,
        uthese_ptr,
        uinfo_ptr,
        uts_ptr,
        sigsetsize,
    )
    .map(|ret| ret as i32)
}

/// Queue a signal and data.
pub unsafe fn rt_tgsigqueueinfo(
    tgid: pid_t,
    tid: pid_t,
    sig: i32,
    uinfo: &mut siginfo_t,
) -> Result<(), Errno> {
    let tgid = tgid as usize;
    let tid = tid as usize;
    let sig = sig as usize;
    let uinfo_ptr = uinfo as *mut siginfo_t as usize;
    syscall4(SYS_RT_TGSIGQUEUEINFO, tgid, tid, sig, uinfo_ptr).map(drop)
}

/// Get a thread's CPU affinity mask.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// const SET_BITS: usize = 16;
/// #[repr(C)]
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct CPUSet {
///     pub bits: [usize; SET_BITS],
/// }
///
/// impl Default for CPUSet {
///     fn default() -> Self {
///         CPUSet {
///             bits: [0; SET_BITS],
///         }
///     }
/// }
///
/// impl CPUSet {
///     #[inline]
///     pub const fn size() -> usize {
///         SET_BITS * size_of::<usize>()
///     }
///
///     #[inline]
///     pub const fn bits_size() -> usize {
///         CPUSet::size() * 8
///     }
///
///     pub fn set(&mut self, pos: usize) -> Result<(), nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         self.bits[bit_pos] |= 1 << (pos % (8 * size_of::<usize>()));
///         Ok(())
///     }
///
///     pub fn clear(&mut self, pos: usize) -> Result<(), nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         self.bits[bit_pos] &= !(1 << (pos % (8 * size_of::<usize>())));
///         Ok(())
///     }
///
///     pub fn is_set(&self, pos: usize) -> Result<bool, nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         let ret = self.bits[bit_pos] & (1 << (pos % (8 * size_of::<usize>())));
///
///         Ok(ret != 0)
///     }
///
///     pub fn as_ptr(&self) -> &[usize] {
///         &self.bits
///     }
///
///     pub fn as_mut_ptr(&mut self) -> &mut [usize] {
///         &mut self.bits
///     }
/// }
///
/// fn main() {
///     let mut set = CPUSet::default();
///     assert!(set.set(1).is_ok());
///     println!("set(1): {:?}", set.is_set(1));
///     assert!(set.set(2).is_ok());
///     assert!(set.clear(2).is_ok());
///     println!("set(2): {:?}", set.is_set(2));
///
///     let ret = unsafe { nc::sched_setaffinity(0, CPUSet::size(), set.as_ptr()) };
///     assert!(ret.is_ok());
///
///     let mut set2 = CPUSet::default();
///     let ret = unsafe { nc::sched_getaffinity(0, CPUSet::size(), set2.as_mut_ptr()) };
///     assert!(ret.is_ok());
///     assert_eq!(set, set2);
/// }
/// ```
pub unsafe fn sched_getaffinity(
    pid: pid_t,
    len: usize,
    user_mask: &mut [usize],
) -> Result<(), Errno> {
    let pid = pid as usize;
    let user_mask_ptr = user_mask.as_mut_ptr() as usize;
    syscall3(SYS_SCHED_GETAFFINITY, pid, len, user_mask_ptr).map(drop)
}

/// Get scheduling policy and attributes
pub unsafe fn sched_getattr(
    pid: pid_t,
    attr: &mut sched_attr_t,
    size: u32,
    flags: u32,
) -> Result<(), Errno> {
    let pid = pid as usize;
    let attr_ptr = attr as *mut sched_attr_t as usize;
    let size = size as usize;
    let flags = flags as usize;
    syscall4(SYS_SCHED_GETATTR, pid, attr_ptr, size, flags).map(drop)
}

/// Get scheduling paramters.
///
/// # Example
///
/// ```
/// let mut param = nc::sched_param_t::default();
/// let ret = unsafe { nc::sched_getparam(0, &mut param) };
/// assert!(ret.is_ok());
/// assert_eq!(param.sched_priority, 0);
/// ```
pub unsafe fn sched_getparam(pid: pid_t, param: &mut sched_param_t) -> Result<(), Errno> {
    let pid = pid as usize;
    let param_ptr = param as *mut sched_param_t as usize;
    syscall2(SYS_SCHED_GETPARAM, pid, param_ptr).map(drop)
}

/// Get scheduling parameter.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::sched_getscheduler(0) };
/// assert_eq!(ret, Ok(nc::SCHED_NORMAL));
/// ```
pub unsafe fn sched_getscheduler(pid: pid_t) -> Result<i32, Errno> {
    let pid = pid as usize;
    syscall1(SYS_SCHED_GETSCHEDULER, pid).map(|ret| ret as i32)
}

/// Get static priority max value.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::sched_get_priority_max(nc::SCHED_RR) };
/// assert!(ret.is_ok());
/// let max_prio = ret.unwrap();
/// assert_eq!(max_prio, 99);
/// ```
pub unsafe fn sched_get_priority_max(policy: i32) -> Result<i32, Errno> {
    let policy = policy as usize;
    syscall1(SYS_SCHED_GET_PRIORITY_MAX, policy).map(|ret| ret as i32)
}

/// Get static priority min value.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::sched_get_priority_min(nc::SCHED_RR) };
/// assert!(ret.is_ok());
/// let min_prio = ret.unwrap();
/// assert_eq!(min_prio, 1);
/// ```
pub unsafe fn sched_get_priority_min(policy: i32) -> Result<i32, Errno> {
    let policy = policy as usize;
    syscall1(SYS_SCHED_GET_PRIORITY_MIN, policy).map(|ret| ret as i32)
}

/// Get the `SCHED_RR` interval for the named process.
///
/// # Example
///
/// ```
/// let mut ts = nc::timespec_t::default();
/// let ret = unsafe { nc::sched_rr_get_interval(0, &mut ts) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn sched_rr_get_interval(pid: pid_t, interval: &mut timespec_t) -> Result<(), Errno> {
    let pid = pid as usize;
    let interval_ptr = interval as *mut timespec_t as usize;
    syscall2(SYS_SCHED_RR_GET_INTERVAL, pid, interval_ptr).map(drop)
}

/// Set a thread's CPU affinity mask.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// const SET_BITS: usize = 16;
/// #[repr(C)]
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// struct CPUSet {
///     pub bits: [usize; SET_BITS],
/// }
///
/// impl Default for CPUSet {
///     fn default() -> Self {
///         CPUSet {
///             bits: [0; SET_BITS],
///         }
///     }
/// }
///
/// impl CPUSet {
///     #[inline]
///     pub const fn size() -> usize {
///         SET_BITS * size_of::<usize>()
///     }
///
///     #[inline]
///     pub const fn bits_size() -> usize {
///         CPUSet::size() * 8
///     }
///
///     pub fn set(&mut self, pos: usize) -> Result<(), nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         self.bits[bit_pos] |= 1 << (pos % (8 * size_of::<usize>()));
///         Ok(())
///     }
///
///     pub fn clear(&mut self, pos: usize) -> Result<(), nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         self.bits[bit_pos] &= !(1 << (pos % (8 * size_of::<usize>())));
///         Ok(())
///     }
///
///     pub fn is_set(&self, pos: usize) -> Result<bool, nc::Errno> {
///         if pos >= CPUSet::bits_size() {
///             return Err(nc::EINVAL);
///         }
///         let bit_pos = pos / 8 / size_of::<usize>();
///         let ret = self.bits[bit_pos] & (1 << (pos % (8 * size_of::<usize>())));
///
///         Ok(ret != 0)
///     }
///
///     pub fn as_ptr(&self) -> &[usize] {
///         &self.bits
///     }
///
///     pub fn as_mut_ptr(&mut self) -> &mut [usize] {
///         &mut self.bits
///     }
/// }
///
/// fn main() {
///     let mut set = CPUSet::default();
///     assert!(set.set(1).is_ok());
///     println!("set(1): {:?}", set.is_set(1));
///     assert!(set.set(2).is_ok());
///     assert!(set.clear(2).is_ok());
///     println!("set(2): {:?}", set.is_set(2));
///
///     let ret = unsafe { nc::sched_setaffinity(0, CPUSet::size(), set.as_ptr()) };
///     assert!(ret.is_ok());
///
///     let mut set2 = CPUSet::default();
///     let ret = unsafe { nc::sched_getaffinity(0, CPUSet::size(), set2.as_mut_ptr()) };
///     assert!(ret.is_ok());
///     assert_eq!(set, set2);
/// }
/// ```
pub unsafe fn sched_setaffinity(pid: pid_t, len: usize, user_mask: &[usize]) -> Result<(), Errno> {
    let pid = pid as usize;
    let user_mask_ptr = user_mask.as_ptr() as usize;
    syscall3(SYS_SCHED_SETAFFINITY, pid, len, user_mask_ptr).map(drop)
}

/// Set the RT priority of a thread.
pub unsafe fn sched_setattr(pid: pid_t, attr: &mut sched_attr_t, flags: u32) -> Result<(), Errno> {
    let pid = pid as usize;
    let attr_ptr = attr as *mut sched_attr_t as usize;
    let flags = flags as usize;
    syscall3(SYS_SCHED_SETATTR, pid, attr_ptr, flags).map(drop)
}

/// Set scheduling paramters.
///
/// # Example
///
/// ```
/// // This call always returns error because default scheduler is SCHED_NORMAL.
/// // We shall call sched_setscheduler() and change to realtime policy
/// // like SCHED_RR or SCHED_FIFO.
/// let sched_param = nc::sched_param_t { sched_priority: 12 };
/// let ret = unsafe { nc::sched_setparam(0, &sched_param) };
/// assert_eq!(ret, Err(nc::EINVAL));
/// ```
pub unsafe fn sched_setparam(pid: pid_t, param: &sched_param_t) -> Result<(), Errno> {
    let pid = pid as usize;
    let param_ptr = param as *const sched_param_t as usize;
    syscall2(SYS_SCHED_SETPARAM, pid, param_ptr).map(drop)
}

/// Set scheduling parameter.
///
/// # Example
///
/// ```
/// let sched_param = nc::sched_param_t { sched_priority: 12 };
/// let ret = unsafe { nc::sched_setscheduler(0, nc::SCHED_RR, &sched_param) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn sched_setscheduler(
    pid: pid_t,
    policy: i32,
    param: &sched_param_t,
) -> Result<(), Errno> {
    let pid = pid as usize;
    let policy = policy as usize;
    let param_ptr = param as *const sched_param_t as usize;
    syscall3(SYS_SCHED_SETSCHEDULER, pid, policy, param_ptr).map(drop)
}

/// Yield the processor.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::sched_yield() };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn sched_yield() -> Result<(), Errno> {
    syscall0(SYS_SCHED_YIELD).map(drop)
}

/// Operate on Secure Computing state of the process.
pub unsafe fn seccomp(operation: u32, flags: u32, args: usize) -> Result<(), Errno> {
    let operation = operation as usize;
    let flags = flags as usize;
    syscall3(SYS_SECCOMP, operation, flags, args).map(drop)
}

/// Sychronous I/O multiplexing.
pub unsafe fn select(
    nfds: i32,
    readfds: &mut fd_set_t,
    writefds: &mut fd_set_t,
    exceptfds: &mut fd_set_t,
    timeout: &mut timeval_t,
) -> Result<i32, Errno> {
    let nfds = nfds as usize;
    let readfds_ptr = readfds as *mut fd_set_t as usize;
    let writefds_ptr = writefds as *mut fd_set_t as usize;
    let exceptfds_ptr = exceptfds as *mut fd_set_t as usize;
    let timeout_ptr = timeout as *mut timeval_t as usize;
    syscall5(
        SYS_SELECT,
        nfds,
        readfds_ptr,
        writefds_ptr,
        exceptfds_ptr,
        timeout_ptr,
    )
    .map(|ret| ret as i32)
}

/// System V semaphore control operations
pub unsafe fn semctl(semid: i32, semnum: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    let semid = semid as usize;
    let semnum = semnum as usize;
    let cmd = cmd as usize;
    syscall4(SYS_SEMCTL, semid, semnum, cmd, arg).map(|ret| ret as i32)
}

/// Get a System V semphore set identifier.
pub unsafe fn semget(key: key_t, nsems: i32, semflg: i32) -> Result<i32, Errno> {
    let key = key as usize;
    let nsems = nsems as usize;
    let semflg = semflg as usize;
    syscall3(SYS_SEMGET, key, nsems, semflg).map(|ret| ret as i32)
}

/// Transfer data between two file descriptors.
pub unsafe fn sendfile(
    out_fd: i32,
    in_fd: i32,
    offset: &mut off_t,
    count: size_t,
) -> Result<ssize_t, Errno> {
    let out_fd = out_fd as usize;
    let in_fd = in_fd as usize;
    let offset_ptr = offset as *mut off_t as usize;
    syscall4(SYS_SENDFILE, out_fd, in_fd, offset_ptr, count).map(|ret| ret as ssize_t)
}

/// Transfer data between file descriptors.
pub unsafe fn sendfile64(
    out_fd: i32,
    in_fd: i32,
    offset: loff_t,
    count: size_t,
) -> Result<ssize_t, Errno> {
    let out_fd = out_fd as usize;
    let in_fd = in_fd as usize;
    let offset = offset as usize;
    let count = count as usize;
    syscall4(SYS_SENDFILE64, out_fd, in_fd, offset, count).map(|ret| ret as ssize_t)
}

/// Send multiple messages on a socket
pub unsafe fn sendmmsg(sockfd: i32, msgvec: &mut [mmsghdr_t], flags: i32) -> Result<i32, Errno> {
    let sockfd = sockfd as usize;
    let msgvec_ptr = (msgvec as *mut [mmsghdr_t]).cast::<*mut mmsghdr_t>() as usize;
    let vlen = msgvec.len();
    let flags = flags as usize;
    syscall4(SYS_SENDMMSG, sockfd, msgvec_ptr, vlen, flags).map(|ret| ret as i32)
}

/// Send a message on a socket. Allow sending ancillary data.
pub unsafe fn sendmsg(sockfd: i32, msg: &msghdr_t, flags: i32) -> Result<ssize_t, Errno> {
    let sockfd = sockfd as usize;
    let msg_ptr = msg as *const msghdr_t as usize;
    let flags = flags as usize;
    syscall3(SYS_SENDMSG, sockfd, msg_ptr, flags).map(|ret| ret as ssize_t)
}

/// Send a message on a socket.
pub unsafe fn sendto(
    sockfd: i32,
    buf: &[u8],
    len: size_t,
    flags: i32,
    dest_addr: &sockaddr_in_t,
    addrlen: socklen_t,
) -> Result<ssize_t, Errno> {
    let sockfd = sockfd as usize;
    let buf_ptr = buf.as_ptr() as usize;
    let flags = flags as usize;
    let dest_addr_ptr = dest_addr as *const sockaddr_in_t as usize;
    let addrlen = addrlen as usize;
    syscall6(
        SYS_SENDTO,
        sockfd,
        buf_ptr,
        len,
        flags,
        dest_addr_ptr,
        addrlen,
    )
    .map(|ret| ret as ssize_t)
}

/// Set NIS domain name.
///
/// # Example
///
/// ```
/// let name = "local-rust-domain";
/// let ret = unsafe { nc::setdomainname(name) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setdomainname<P: AsRef<Path>>(name: P) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let name_len = name.len();
    syscall2(SYS_SETDOMAINNAME, name_ptr, name_len).map(drop)
}

/// Set group identify used for filesystem checkes.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setfsgid(0) };
/// assert!(ret.is_ok());
/// let gid = unsafe { nc::getgid() };
/// assert_eq!(ret, Ok(gid));
/// ```
pub unsafe fn setfsgid(fsgid: gid_t) -> Result<gid_t, Errno> {
    let fsgid = fsgid as usize;
    syscall1(SYS_SETFSGID, fsgid).map(|ret| ret as gid_t)
}

/// Set user identify used for filesystem checkes.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setfsuid(0) };
/// assert!(ret.is_ok());
/// let uid = unsafe { nc::getuid() };
/// assert_eq!(ret, Ok(uid));
/// ```
pub unsafe fn setfsuid(fsuid: uid_t) -> Result<uid_t, Errno> {
    let fsuid = fsuid as usize;
    syscall1(SYS_SETFSUID, fsuid).map(|ret| ret as uid_t)
}

/// Set the group ID of the calling process to `gid`.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setgid(0) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setgid(gid: gid_t) -> Result<(), Errno> {
    let gid = gid as usize;
    syscall1(SYS_SETGID, gid).map(drop)
}

/// Set list of supplementary group Ids.
///
/// # Example
///
/// ```
/// let list = [0, 1, 2];
/// let ret = unsafe { nc::setgroups(&list) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setgroups(group_list: &[gid_t]) -> Result<(), Errno> {
    let group_len = group_list.len();
    let group_ptr = group_list.as_ptr() as usize;
    syscall2(SYS_SETGROUPS, group_len, group_ptr).map(drop)
}

/// Set hostname.
///
/// # Example
///
/// ```
/// let name = "rust-machine";
/// let ret = unsafe { nc::sethostname(name) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn sethostname<P: AsRef<Path>>(name: P) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let name_len = name.len();
    syscall2(SYS_SETHOSTNAME, name_ptr, name_len).map(drop)
}

/// Set value of an interval timer.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
///     let msg = "Hello alarm";
///     let _ = unsafe { nc::write(2, msg.as_ptr() as usize, msg.len()) };
/// }
///
/// let sa = nc::sigaction_t {
///     sa_handler: handle_alarm as nc::sighandler_t,
///     sa_flags: 0,
///     ..nc::sigaction_t::default()
/// };
/// let mut old_sa = nc::sigaction_t::default();
/// let ret = unsafe { nc::rt_sigaction(nc::SIGALRM, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
/// assert!(ret.is_ok());
///
/// // Single shot timer, actived after 1 second.
/// let itv = nc::itimerval_t {
///     it_value: nc::timeval_t {
///         tv_sec: 1,
///         tv_usec: 0,
///     },
///     it_interval: nc::timeval_t {
///         tv_sec: 0,
///         tv_usec: 0,
///     },
/// };
/// let mut prev_itv = nc::itimerval_t::default();
/// let ret = unsafe { nc::setitimer(nc::ITIMER_REAL, &itv, &mut prev_itv) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::getitimer(nc::ITIMER_REAL, &mut prev_itv) };
/// assert!(ret.is_ok());
/// assert!(prev_itv.it_value.tv_sec <= itv.it_value.tv_sec);
///
/// let mask = nc::sigset_t::default();
/// let _ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///
/// let ret = unsafe { nc::getitimer(nc::ITIMER_REAL, &mut prev_itv) };
/// assert!(ret.is_ok());
/// assert_eq!(prev_itv.it_value.tv_sec, 0);
/// assert_eq!(prev_itv.it_value.tv_usec, 0);
/// ```
pub unsafe fn setitimer(
    which: i32,
    new_val: &itimerval_t,
    old_val: &mut itimerval_t,
) -> Result<(), Errno> {
    let which = which as usize;
    let new_val_ptr = new_val as *const itimerval_t as usize;
    let old_val_ptr = old_val as *mut itimerval_t as usize;
    syscall3(SYS_SETITIMER, which, new_val_ptr, old_val_ptr).map(drop)
}

/// Reassociate thread with a namespace.
pub unsafe fn setns(fd: i32, nstype: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    let nstype = nstype as usize;
    syscall2(SYS_SETNS, fd, nstype).map(drop)
}

/// Set the process group ID (PGID) of the process specified by `pid` to `pgid`.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setpgid(nc::getpid(), 1) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> Result<(), Errno> {
    let pid = pid as usize;
    let pgid = pgid as usize;
    syscall2(SYS_SETPGID, pid, pgid).map(drop)
}

/// Set program scheduling priority.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setpriority(nc::PRIO_PROCESS, nc::getpid(), -19) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EACCES))
/// ```
pub unsafe fn setpriority(which: i32, who: i32, prio: i32) -> Result<(), Errno> {
    let which = which as usize;
    let who = who as usize;
    let prio = prio as usize;
    syscall3(SYS_SETPRIORITY, which, who, prio).map(drop)
}

/// Set real and effective group IDs of the calling process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setregid(0, 0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setregid(rgid: gid_t, egid: gid_t) -> Result<(), Errno> {
    let rgid = rgid as usize;
    let egid = egid as usize;
    syscall2(SYS_SETREGID, rgid, egid).map(drop)
}

/// Set real, effective and saved group Ids of the calling process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setresgid(0, 0, 0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> Result<(), Errno> {
    let rgid = rgid as usize;
    let egid = egid as usize;
    let sgid = sgid as usize;
    syscall3(SYS_SETRESGID, rgid, egid, sgid).map(drop)
}

/// Set real, effective and saved user Ids of the calling process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setresuid(0, 0, 0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> Result<(), Errno> {
    let ruid = ruid as usize;
    let euid = euid as usize;
    let suid = suid as usize;
    syscall3(SYS_SETRESUID, ruid, euid, suid).map(drop)
}

/// Set real and effective user IDs of the calling process.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setreuid(0, 0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> Result<(), Errno> {
    let ruid = ruid as usize;
    let euid = euid as usize;
    syscall2(SYS_SETREUID, ruid, euid).map(drop)
}

/// Set resource limit.
///
/// # Example
///
/// ```
/// let rlimit = nc::rlimit_t {
///     rlim_cur: 128,
///     rlim_max: 128,
/// };
/// let ret = unsafe { nc::setrlimit(nc::RLIMIT_NOFILE, &rlimit) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn setrlimit(resource: i32, rlimit: &rlimit_t) -> Result<(), Errno> {
    let resource = resource as usize;
    let rlimit_ptr = rlimit as *const rlimit_t as usize;
    syscall2(SYS_SETRLIMIT, resource, rlimit_ptr).map(drop)
}

/// Create a new session if the calling process is not a process group leader.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setsid() };
/// assert!(ret.is_ok());
/// let pid = unsafe { nc::getpid() };
/// assert_eq!(ret, Ok(pid));
/// ```
pub unsafe fn setsid() -> Result<pid_t, Errno> {
    syscall0(SYS_SETSID).map(|ret| ret as pid_t)
}

/// Set options on sockets.
///
/// # Example
///
/// ```
/// let socket_fd = unsafe { nc::socket(nc::AF_INET, nc::SOCK_STREAM, 0) };
/// assert!(socket_fd.is_ok());
/// let socket_fd = socket_fd.unwrap();
///
/// // Enable tcp fast open.
/// let queue_len: i32 = 5;
/// let queue_len_ptr = &queue_len as *const i32 as usize;
/// let ret = unsafe {
///     nc::setsockopt(
///         socket_fd,
///         nc::IPPROTO_TCP,
///         nc::TCP_FASTOPEN,
///         queue_len_ptr,
///         std::mem::size_of_val(&queue_len) as u32,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(socket_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn setsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: usize,
    optlen: socklen_t,
) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let level = level as usize;
    let optname = optname as usize;
    let optlen = optlen as usize;
    syscall5(SYS_SETSOCKOPT, sockfd, level, optname, optval, optlen).map(drop)
}

/// Set system time and timezone.
///
/// ```
/// let tv = nc::timeval_t {
///     tv_sec: 0,
///     tv_usec: 0,
/// };
/// let tz = nc::timezone_t::default();
/// let ret = unsafe { nc::settimeofday(&tv, &tz) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn settimeofday(timeval: &timeval_t, tz: &timezone_t) -> Result<(), Errno> {
    let timeval_ptr = timeval as *const timeval_t as usize;
    let tz_ptr = tz as *const timezone_t as usize;
    syscall2(SYS_SETTIMEOFDAY, timeval_ptr, tz_ptr).map(drop)
}

/// Set the effective user ID of the calling process to `uid`.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::setuid(0) };
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn setuid(uid: uid_t) -> Result<(), Errno> {
    let uid = uid as usize;
    syscall1(SYS_SETUID, uid).map(drop)
}

/// Set extended attribute value.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-setxattr";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let attr_name = "user.creator";
/// let attr_value = "nc-0.0.1";
/// //let flags = 0;
/// let flags = nc::XATTR_CREATE;
/// let ret = unsafe {
///     nc::setxattr(
///         path,
///         &attr_name,
///         attr_value.as_ptr() as usize,
///         attr_value.len(),
///         flags,
///     )
/// };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn setxattr<P: AsRef<Path>>(
    filename: P,
    name: P,
    value: usize,
    size: size_t,
    flags: i32,
) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall5(SYS_SETXATTR, filename_ptr, name_ptr, value, size, flags).map(drop)
}

/// Set default NUMA memory policy for a thread and its children
pub unsafe fn set_mempolicy(mode: i32, nmask: *const usize, maxnode: usize) -> Result<(), Errno> {
    let mode = mode as usize;
    let nmask = nmask as usize;
    syscall3(SYS_SET_MEMPOLICY, mode, nmask, maxnode).map(drop)
}

/// Set the robust-futex list head of a task.
pub unsafe fn set_robust_list(heads: &mut [robust_list_head_t]) -> Result<(), Errno> {
    let heads_ptr = heads.as_mut_ptr() as usize;
    let len = heads.len();
    syscall2(SYS_SET_ROBUST_LIST, heads_ptr, len).map(drop)
}

/// Set thread-local storage information.
pub unsafe fn set_thread_area(addr: usize) -> Result<(), Errno> {
    syscall1(SYS_SET_THREAD_AREA, addr).map(drop)
}

/// Set pointer to thread ID.
pub unsafe fn set_tid_address(tid: &mut i32) -> Result<isize, Errno> {
    let tid_ptr = tid as *mut i32 as usize;
    syscall1(SYS_SET_TID_ADDRESS, tid_ptr).map(|ret| ret as isize)
}

/// Attach the System V shared memory segment.
///
/// # Example
///
/// ```
/// let size = 4 * nc::PAGE_SIZE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | 0o600;
/// let ret = unsafe { nc::shmget(nc::IPC_PRIVATE, size, flags) };
/// assert!(ret.is_ok());
/// let shmid = ret.unwrap();
///
/// let addr: usize = 0;
/// let ret = unsafe { nc::shmat(shmid, addr, 0) };
/// assert!(ret.is_ok());
/// let addr = ret.unwrap();
///
/// let mut buf = nc::shmid_ds_t::default();
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_STAT, &mut buf) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmdt(addr) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn shmat(shmid: i32, shmaddr: usize, shmflg: i32) -> Result<usize, Errno> {
    let shmid = shmid as usize;
    let shmflg = shmflg as usize;
    syscall3(SYS_SHMAT, shmid, shmaddr, shmflg)
}

/// System V shared memory control.
///
/// # Example
///
/// ```
/// let size = 4 * nc::PAGE_SIZE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | 0o600;
/// let ret = unsafe { nc::shmget(nc::IPC_PRIVATE, size, flags) };
/// assert!(ret.is_ok());
/// let shmid = ret.unwrap();
/// let mut buf = nc::shmid_ds_t::default();
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn shmctl(shmid: i32, cmd: i32, buf: &mut shmid_ds_t) -> Result<i32, Errno> {
    let shmid = shmid as usize;
    let cmd = cmd as usize;
    let buf_ptr = buf as *mut shmid_ds_t as usize;
    syscall3(SYS_SHMCTL, shmid, cmd, buf_ptr).map(|ret| ret as i32)
}

/// Detach the System V shared memory segment.
///
/// # Example
///
/// ```
/// let size = 4 * nc::PAGE_SIZE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | 0o600;
/// let ret = unsafe { nc::shmget(nc::IPC_PRIVATE, size, flags) };
/// assert!(ret.is_ok());
/// let shmid = ret.unwrap();
///
/// let addr: usize = 0;
/// let ret = unsafe { nc::shmat(shmid, addr, 0) };
/// assert!(ret.is_ok());
/// let addr = ret.unwrap();
///
/// let mut buf = nc::shmid_ds_t::default();
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_STAT, &mut buf) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmdt(addr) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn shmdt(shmaddr: usize) -> Result<(), Errno> {
    syscall1(SYS_SHMDT, shmaddr).map(drop)
}

/// Allocates a System V shared memory segment.
///
/// # Example
///
/// ```
/// let size = 4 * nc::PAGE_SIZE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | 0o600;
/// let ret = unsafe { nc::shmget(nc::IPC_PRIVATE, size, flags) };
/// assert!(ret.is_ok());
/// let _shmid = ret.unwrap();
/// ```
pub unsafe fn shmget(key: key_t, size: size_t, shmflg: i32) -> Result<i32, Errno> {
    let key = key as usize;
    let shmflg = shmflg as usize;
    syscall3(SYS_SHMGET, key, size, shmflg).map(|ret| ret as i32)
}

/// Shutdown part of a full-duplex connection.
pub unsafe fn shutdown(sockfd: i32, how: i32) -> Result<(), Errno> {
    let sockfd = sockfd as usize;
    let how = how as usize;
    syscall2(SYS_SHUTDOWN, sockfd, how).map(drop)
}

/// Examine and change a signal action.
pub unsafe fn sigaction(
    sig: i32,
    act: &sigaction_t,
    old_act: &mut sigaction_t,
) -> Result<(), Errno> {
    let sig = sig as usize;
    let act_ptr = act as *const sigaction_t as usize;
    let old_act_ptr = old_act as *mut sigaction_t as usize;
    syscall3(SYS_SIGACTION, sig, act_ptr, old_act_ptr).map(drop)
}

/// Get/set signal stack context.
pub unsafe fn sigaltstack(uss: &sigaltstack_t, uoss: &mut sigaltstack_t) -> Result<(), Errno> {
    let uss_ptr = uss as *const sigaltstack_t as usize;
    let uoss_ptr = uoss as *mut sigaltstack_t as usize;
    syscall2(SYS_SIGALTSTACK, uss_ptr, uoss_ptr).map(drop)
}

/// Signal handling.
///
/// Deprecated. Use sigaction() instead.
///
/// # Example
///
/// ```
/// fn handle_sigterm(signum: i32) {
///     assert_eq!(signum, nc::SIGTERM);
/// }
/// // let ret = nc::signal(nc::SIGTERM, nc::SIG_IGN);
/// let ret = unsafe { nc::signal(nc::SIGTERM, handle_sigterm as nc::sighandler_t) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::kill(nc::getpid(), nc::SIGTERM) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn signal(sig: i32, handler: sighandler_t) -> Result<sighandler_t, Errno> {
    let sig = sig as usize;
    let handler = handler as usize;
    syscall2(SYS_SIGNAL, sig, handler).map(|ret| ret as sighandler_t)
}

/// Create a file descriptor to accept signals.
pub unsafe fn signalfd(fd: i32, mask: &[sigset_t]) -> Result<i32, Errno> {
    let fd = fd as usize;
    let mask_ptr = mask.as_ptr() as usize;
    let mask_len = mask.len();
    syscall3(SYS_SIGNALFD, fd, mask_ptr, mask_len).map(|ret| ret as i32)
}

/// Create a file descriptor to accept signals.
pub unsafe fn signalfd4(fd: i32, mask: &[sigset_t], flags: i32) -> Result<i32, Errno> {
    let fd = fd as usize;
    let mask_ptr = mask.as_ptr() as usize;
    let mask_len = mask.len();
    let flags = flags as usize;
    syscall4(SYS_SIGNALFD4, fd, mask_ptr, mask_len, flags).map(|ret| ret as i32)
}

/// Examine pending signals.
pub unsafe fn sigpending(set: &mut sigset_t) -> Result<(), Errno> {
    let set_ptr = set as *mut sigset_t as usize;
    syscall1(SYS_SIGPENDING, set_ptr).map(drop)
}

/// Examine and change blocked signals.
pub unsafe fn sigprocmask(
    how: i32,
    newset: &mut sigset_t,
    oldset: &mut sigset_t,
) -> Result<(), Errno> {
    let how = how as usize;
    let newset_ptr = newset as *mut sigset_t as usize;
    let oldset_ptr = oldset as *mut sigset_t as usize;
    syscall3(SYS_SIGPROCMASK, how, newset_ptr, oldset_ptr).map(drop)
}

/// Return from signal handler and cleanup stack frame.
/// Never returns.
pub unsafe fn sigreturn() {
    let _ = syscall0(SYS_SIGRETURN);
}

/// Wait for a signal.
///
/// # Example
/// ```
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
///
/// if pid == 0 {
///     // child process.
///     let mask = nc::sigset_t::default();
///     let ret = unsafe { nc::sigsuspend(&mask) };
///     assert!(ret.is_ok());
/// } else {
///     // parent process.
///     let t = nc::timespec_t {
///         tv_sec: 1,
///         tv_nsec: 0,
///     };
///     let ret = unsafe { nc::nanosleep(&t, None) };
///     assert!(ret.is_ok());
///
///     let ret = unsafe { nc::kill(pid, nc::SIGTERM) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn sigsuspend(mask: &old_sigset_t) -> Result<(), Errno> {
    let mask_ptr = mask as *const old_sigset_t as usize;
    syscall1(SYS_SIGSUSPEND, mask_ptr).map(drop)
}

/// Create an endpoint for communication.
///
/// # Example
///
/// ```
/// let socket_fd = unsafe { nc::socket(nc::AF_INET, nc::SOCK_STREAM, 0) };
/// assert!(socket_fd.is_ok());
/// let socket_fd = socket_fd.unwrap();
/// let ret = unsafe { nc::close(socket_fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn socket(domain: i32, sock_type: i32, protocol: i32) -> Result<i32, Errno> {
    let domain = domain as usize;
    let sock_type = sock_type as usize;
    let protocol = protocol as usize;
    syscall3(SYS_SOCKET, domain, sock_type, protocol).map(|ret| ret as i32)
}

/// System call vectors.
///
/// Argument checking cleaned up. Saved 20% in size.
/// This function doesn't need to set the kernel lock because
/// it is set by the callees.
// TODO(Shaohua): Check args type and return type
pub unsafe fn socketcall(call: i32, args: &mut usize) -> Result<usize, Errno> {
    let call = call as usize;
    let args_ptr = args as *mut usize as usize;
    syscall2(SYS_SOCKETCALL, call, args_ptr)
}

/// Create a pair of connected socket.
pub unsafe fn socketpair(
    domain: i32,
    type_: i32,
    protocol: i32,
    sv: [i32; 2],
) -> Result<(), Errno> {
    let domain = domain as usize;
    let type_ = type_ as usize;
    let protocol = protocol as usize;
    let sv_ptr = sv.as_ptr() as usize;
    syscall4(SYS_SOCKETPAIR, domain, type_, protocol, sv_ptr).map(drop)
}

/// Splice data to/from pipe.
///
/// # Example
///
/// ```
/// let mut fds_left = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds_left, 0) };
/// assert!(ret.is_ok());
///
/// let mut fds_right = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds_right, 0) };
/// assert!(ret.is_ok());
///
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fds_left[1], msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// let n_write = ret.unwrap() as nc::size_t;
/// assert_eq!(n_write, msg.len());
///
/// let ret = unsafe {
///     nc::splice(
///         fds_left[0],
///         None,
///         fds_right[1],
///         None,
///         n_write,
///         nc::SPLICE_F_MOVE,
///     )
/// };
/// assert!(ret.is_ok());
///
/// let mut buf = [0u8; 64];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::read(fds_right[0], buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap() as nc::size_t;
/// assert_eq!(n_read, n_write);
/// let read_msg = std::str::from_utf8(&buf[..n_read]);
/// assert!(read_msg.is_ok());
/// assert_eq!(Ok(msg), read_msg);
///
/// unsafe {
///     assert!(nc::close(fds_left[0]).is_ok());
///     assert!(nc::close(fds_left[1]).is_ok());
///     assert!(nc::close(fds_right[0]).is_ok());
///     assert!(nc::close(fds_right[1]).is_ok());
/// }
/// ```
pub unsafe fn splice(
    fd_in: i32,
    off_in: Option<&mut loff_t>,
    fd_out: i32,
    off_out: Option<&mut loff_t>,
    len: size_t,
    flags: u32,
) -> Result<ssize_t, Errno> {
    let fd_in = fd_in as usize;
    let off_in_ptr = off_in.map_or(0, |off_in| off_in as *mut loff_t as usize);
    let fd_out = fd_out as usize;
    let off_out_ptr = off_out.map_or(0, |off_out| off_out as *mut loff_t as usize);
    let flags = flags as usize;
    syscall6(
        SYS_SPLICE,
        fd_in,
        off_in_ptr,
        fd_out,
        off_out_ptr,
        len,
        flags,
    )
    .map(|ret| ret as ssize_t)
}

/// Get file status about a file.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat_t::default();
/// let ret = unsafe { nc::stat(path, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn stat<P: AsRef<Path>>(filename: P, statbuf: &mut stat_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat_t as usize;
    syscall2(SYS_STAT, filename_ptr, statbuf_ptr).map(drop)
}

/// Get file status about a file.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut stat = nc::stat64_t::default();
/// let ret = unsafe { nc::stat64(path, &mut stat) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((stat.st_mode & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn stat64<P: AsRef<Path>>(filename: P, statbuf: &mut stat64_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let statbuf_ptr = statbuf as *mut stat64_t as usize;
    syscall2(SYS_STAT64, filename_ptr, statbuf_ptr).map(drop)
}

/// Get filesystem statistics.
///
/// # Example
///
/// ```
/// let path = "/usr";
/// let mut statfs = nc::statfs_t::default();
/// let ret = unsafe { nc::statfs(path, &mut statfs) };
/// assert!(ret.is_ok());
/// assert!(statfs.f_bfree > 0);
/// assert!(statfs.f_bavail > 0);
/// ```
pub unsafe fn statfs<P: AsRef<Path>>(filename: P, buf: &mut statfs_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let buf_ptr = buf as *mut statfs_t as usize;
    syscall2(SYS_STATFS, filename_ptr, buf_ptr).map(drop)
}

/// Get filesystem statistics.
///
/// # Example
///
/// ```
/// let path = "/usr";
/// let mut statfs = nc::statfs64_t::default();
/// let ret = unsafe { nc::statfs64(path, &mut statfs) };
/// assert!(ret.is_ok());
/// assert!(statfs.f_bfree > 0);
/// assert!(statfs.f_bavail > 0);
/// ```
pub unsafe fn statfs64<P: AsRef<Path>>(filename: P, buf: &mut statfs64_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let buf_ptr = buf as *mut statfs64_t as usize;
    syscall2(SYS_STATFS64, filename_ptr, buf_ptr).map(drop)
}

/// Get file status about a file (extended).
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let mut statx = nc::statx_t::default();
/// let ret = unsafe { nc::statx(nc::AT_FDCWD, path, nc::AT_SYMLINK_NOFOLLOW, nc::STATX_TYPE, &mut statx) };
/// assert!(ret.is_ok());
/// // Check fd is a regular file.
/// assert_eq!((statx.stx_mode as u32 & nc::S_IFMT), nc::S_IFREG);
/// ```
pub unsafe fn statx<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    flags: i32,
    mask: u32,
    buf: &mut statx_t,
) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flags = flags as usize;
    let mask = mask as usize;
    let buf_ptr = buf as *mut statx_t as usize;
    syscall5(SYS_STATX, dirfd, filename_ptr, flags, mask, buf_ptr).map(drop)
}

/// Set time.
///
/// # Example
///
/// ```
/// let t = 1611630530;
/// let ret = unsafe { nc::stime(t) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn stime(t: &time_t) -> Result<(), Errno> {
    let t_ptr = t as *const time_t as usize;
    syscall1(SYS_STIME, t_ptr).map(drop)
}

/// Stop swapping to file/device.
///
/// # Example
///
/// ```
/// let filename = "/dev/sda-no-exist";
/// let ret = unsafe { nc::swapoff(filename) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn swapoff<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_SWAPOFF, filename_ptr).map(drop)
}

/// Start swapping to file/device.
///
/// # Example
///
/// ```
/// let filename = "/dev/sda-no-exist";
/// let ret = unsafe { nc::swapon(filename, nc::SWAP_FLAG_PREFER) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
/// ```
pub unsafe fn swapon<P: AsRef<Path>>(filename: P, flags: i32) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_SWAPON, filename_ptr, flags).map(drop)
}

/// Make a new name for a file.
///
/// # Example
///
/// ```
/// let oldname = "/etc/passwd";
/// let newname = "/tmp/nc-symlink";
/// let ret = unsafe { nc::symlink(oldname, newname) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, newname,0 ) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn symlink<P: AsRef<Path>>(oldname: P, newname: P) -> Result<(), Errno> {
    let oldname = PathBuf::new(oldname);
    let oldname_ptr = oldname.as_ptr() as usize;
    let newname = PathBuf::new(newname);
    let newname_ptr = newname.as_ptr() as usize;
    syscall2(SYS_SYMLINK, oldname_ptr, newname_ptr).map(drop)
}

/// Make a new name for a file.
///
/// # Example
///
/// ```
/// let oldname = "/etc/passwd";
/// let newname = "/tmp/nc-symlinkat";
/// let ret = unsafe { nc::symlinkat(oldname, nc::AT_FDCWD, newname) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, newname, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn symlinkat<P: AsRef<Path>>(
    oldname: P,
    newdirfd: i32,
    newname: P,
) -> Result<(), Errno> {
    let oldname = PathBuf::new(oldname);
    let oldname_ptr = oldname.as_ptr() as usize;
    let newname = PathBuf::new(newname);
    let newname_ptr = newname.as_ptr() as usize;
    let newdirfd = newdirfd as usize;
    syscall3(SYS_SYMLINKAT, oldname_ptr, newdirfd, newname_ptr).map(drop)
}

/// Commit filesystem caches to disk.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::sync() };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn sync() -> Result<(), Errno> {
    syscall0(SYS_SYNC).map(drop)
}

/// Commit filesystem cache related to `fd` to disk.
///
/// # Example
///
/// ```
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe {nc::syncfs(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn syncfs(fd: i32) -> Result<(), Errno> {
    let fd = fd as usize;
    syscall1(SYS_SYNCFS, fd).map(drop)
}

/// Sync a file segment to disk
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-sync-file-range";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// let n_write = ret.unwrap();
/// assert_eq!(n_write, msg.len() as nc::ssize_t);
///
/// let ret = unsafe {
///     nc::sync_file_range(
///         fd,
///         0,
///         n_write,
///         nc::SYNC_FILE_RANGE_WAIT_BEFORE
///         | nc::SYNC_FILE_RANGE_WRITE
///         | nc::SYNC_FILE_RANGE_WAIT_AFTER,
///     )
/// };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn sync_file_range(
    fd: i32,
    offset: off_t,
    nbytes: off_t,
    flags: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let offset = offset as usize;
    let nbytes = nbytes as usize;
    let flags = flags as usize;
    syscall4(SYS_SYNC_FILE_RANGE, fd, offset, nbytes, flags).map(drop)
}

/// Get filesystem type information.
pub unsafe fn sysfs(option: i32, arg1: usize, arg2: usize) -> Result<i32, Errno> {
    let option = option as usize;
    syscall3(SYS_SYSFS, option, arg1, arg2).map(|ret| ret as i32)
}

/// Return system information.
///
/// # Example
///
/// ```
/// let mut info = nc::sysinfo_t::default();
/// let ret = unsafe { nc::sysinfo(&mut info) };
/// assert!(ret.is_ok());
/// assert!(info.uptime > 0);
/// assert!(info.freeram > 0);
/// ```
pub unsafe fn sysinfo(info: &mut sysinfo_t) -> Result<(), Errno> {
    let info_ptr = info as *mut sysinfo_t as usize;
    syscall1(SYS_SYSINFO, info_ptr).map(drop)
}

/// Read and/or clear kernel message ring buffer.
pub unsafe fn syslog(action: i32, buf: &mut [u8]) -> Result<i32, Errno> {
    let action = action as usize;
    let buf_ptr = buf.as_mut_ptr() as usize;
    let buf_len = buf.len();
    syscall3(SYS_SYSLOG, action, buf_ptr, buf_len).map(|ret| ret as i32)
}

/// Duplicate pipe content.
///
/// # Example
///
/// ```
/// let mut fds_left = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds_left, 0) };
/// assert!(ret.is_ok());
///
/// let mut fds_right = [0, 0];
/// let ret = unsafe { nc::pipe2(&mut fds_right, 0) };
/// assert!(ret.is_ok());
///
/// let msg = "Hello, Rust";
/// let ret = unsafe { nc::write(fds_left[1], msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// let n_write = ret.unwrap() as nc::size_t;
/// assert_eq!(n_write, msg.len());
///
/// let ret = unsafe { nc::tee(fds_left[0], fds_right[1], n_write, nc::SPLICE_F_NONBLOCK) };
/// assert!(ret.is_ok());
///
/// let mut buf = [0u8; 64];
/// let buf_len = buf.len();
/// let ret = unsafe { nc::read(fds_right[0], buf.as_mut_ptr() as usize, buf_len) };
/// assert!(ret.is_ok());
/// let n_read = ret.unwrap() as nc::size_t;
/// assert_eq!(n_read, n_write);
/// let read_msg = std::str::from_utf8(&buf[..n_read]);
/// assert!(read_msg.is_ok());
/// assert_eq!(Ok(msg), read_msg);
///
/// unsafe {
///     assert!(nc::close(fds_left[0]).is_ok());
///     assert!(nc::close(fds_left[1]).is_ok());
///     assert!(nc::close(fds_right[0]).is_ok());
///     assert!(nc::close(fds_right[1]).is_ok());
/// }
/// ```
pub unsafe fn tee(fd_in: i32, fd_out: i32, len: size_t, flags: u32) -> Result<ssize_t, Errno> {
    let fd_in = fd_in as usize;
    let fd_out = fd_out as usize;
    let flags = flags as usize;
    syscall4(SYS_TEE, fd_in, fd_out, len, flags).map(|ret| ret as ssize_t)
}

/// Send a signal to a thread.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
///
/// if pid == 0 {
///     // child process.
///     let mask = nc::sigset_t::default();
///     let ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
/// } else {
///     // parent process.
///     let t = nc::timespec_t {
///         tv_sec: 1,
///         tv_nsec: 0,
///     };
///     let ret = unsafe { nc::nanosleep(&t, None) };
///     assert!(ret.is_ok());
///
///     let ret = unsafe { nc::tgkill(pid, pid, nc::SIGTERM) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn tgkill(tgid: i32, tid: i32, sig: i32) -> Result<(), Errno> {
    let tgid = tgid as usize;
    let tid = tid as usize;
    let sig = sig as usize;
    syscall3(SYS_TGKILL, tgid, tid, sig).map(drop)
}

/// Get time in seconds.
///
/// # Example
///
/// ```
/// let mut t = 0;
/// let ret = unsafe { nc::time(&mut t) };
/// assert_eq!(ret.unwrap(), t);
/// assert!(t > 1610421040);
/// ```
pub unsafe fn time(t: &mut time_t) -> Result<time_t, Errno> {
    syscall1(SYS_TIME, t as *mut time_t as usize).map(|ret| ret as time_t)
}

/// Create a timer that notifies via a file descriptor.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::timerfd_create(nc::CLOCK_MONOTONIC, nc::TFD_CLOEXEC) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn timerfd_create(clockid: i32, flags: i32) -> Result<i32, Errno> {
    let clockid = clockid as usize;
    let flags = flags as usize;
    syscall2(SYS_TIMERFD_CREATE, clockid, flags).map(|ret| ret as i32)
}

/// Get current timer via a file descriptor.
pub unsafe fn timerfd_gettime(ufd: i32, cur_value: &mut itimerspec_t) -> Result<(), Errno> {
    let ufd = ufd as usize;
    let cur_value_ptr = cur_value as *mut itimerspec_t as usize;
    syscall2(SYS_TIMERFD_GETTIME, ufd, cur_value_ptr).map(drop)
}

/// Set current timer via a file descriptor.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::timerfd_create(nc::CLOCK_MONOTONIC, nc::TFD_CLOEXEC) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
///
/// let flags = 0;
/// let time = nc::itimerspec_t {
///     it_interval: nc::timespec_t::default(),
///     it_value: nc::timespec_t {
///         tv_sec: 1,
///         tv_nsec: 0,
///     },
/// };
/// let ret = unsafe { nc::timerfd_settime(fd, flags, &time, None) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn timerfd_settime(
    ufd: i32,
    flags: i32,
    new_value: &itimerspec_t,
    old_value: Option<&mut itimerspec_t>,
) -> Result<(), Errno> {
    let ufd = ufd as usize;
    let flags = flags as usize;
    let new_value_ptr = new_value as *const itimerspec_t as usize;
    let old_value_ptr = old_value.map_or(0, |old_value| old_value as *mut itimerspec_t as usize);
    syscall4(
        SYS_TIMERFD_SETTIME,
        ufd,
        flags,
        new_value_ptr,
        old_value_ptr,
    )
    .map(drop)
}

/// Create a per-process timer
///
/// # Example
///
/// ```
/// let mut timerid = nc::timer_t::default();
/// let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, None, &mut timerid) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn timer_create(
    clock: clockid_t,
    event: Option<&mut sigevent_t>,
    timer_id: &mut timer_t,
) -> Result<(), Errno> {
    let clock = clock as usize;
    let event_ptr = event.map_or(0, |event| event as *mut sigevent_t as usize);
    let timer_id_ptr = timer_id as *mut timer_t as usize;
    syscall3(SYS_TIMER_CREATE, clock, event_ptr, timer_id_ptr).map(drop)
}

/// Delete a per-process timer
///
/// # Example
///
/// ```
/// let mut timer_id = nc::timer_t::default();
/// let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, None, &mut timer_id) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::timer_delete(timer_id) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn timer_delete(timer_id: timer_t) -> Result<(), Errno> {
    let timer_id = timer_id as usize;
    syscall1(SYS_TIMER_DELETE, timer_id).map(drop)
}

/// Get overrun count for a per-process timer.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
/// }
///
/// fn main() {
///     const TIMER_SIG: i32 = nc::SIGRTMAX;
///
///     let sa = nc::sigaction_t {
///         sa_flags: nc::SA_SIGINFO,
///         sa_handler: handle_alarm as nc::sighandler_t,
///         ..nc::sigaction_t::default()
///     };
///     let mut old_sa = nc::sigaction_t::default();
///     let ret = unsafe { nc::rt_sigaction(TIMER_SIG, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
///
///     let tid = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let mut ev = nc::sigevent_t {
///         sigev_value: nc::sigval_t {
///             sival_ptr: &tid as *const nc::itimerspec_t as usize,
///         },
///         sigev_signo: TIMER_SIG,
///         sigev_notify: nc::SIGEV_SIGNAL,
///         sigev_un: nc::sigev_un_t::default(),
///     };
///     let mut timer_id = nc::timer_t::default();
///     let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, Some(&mut ev), &mut timer_id) };
///     assert!(ret.is_ok());
///     println!("timer id: {:?}", timer_id);
///
///     let flags = 0;
///     let time = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let ret = unsafe { nc::timer_settime(timer_id, flags, &time, None) };
///     assert!(ret.is_ok());
///
///     let mut cur_time = nc::itimerspec_t::default();
///     let ret = unsafe { nc::timer_gettime(timer_id, &mut cur_time) };
///     assert!(ret.is_ok());
///     println!("cur time: {:?}", cur_time);
///
///     let mask = nc::sigset_t::default();
///     let _ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///
///     let ret = unsafe { nc::timer_getoverrun(timer_id) };
///     assert!(ret.is_ok());
///     assert_eq!(ret, Ok(0));
///
///     let ret = unsafe { nc::timer_delete(timer_id) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn timer_getoverrun(timer_id: timer_t) -> Result<i32, Errno> {
    let timer_id = timer_id as usize;
    syscall1(SYS_TIMER_GETOVERRUN, timer_id).map(|ret| ret as i32)
}

/// Fetch state of per-process timer>
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
/// }
///
/// fn main() {
///     const TIMER_SIG: i32 = nc::SIGRTMAX;
///
///     let sa = nc::sigaction_t {
///         sa_flags: nc::SA_SIGINFO,
///         sa_handler: handle_alarm as nc::sighandler_t,
///         ..nc::sigaction_t::default()
///     };
///     let mut old_sa = nc::sigaction_t::default();
///     let ret = unsafe { nc::rt_sigaction(TIMER_SIG, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
///
///     let tid = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let mut ev = nc::sigevent_t {
///         sigev_value: nc::sigval_t {
///             sival_ptr: &tid as *const nc::itimerspec_t as usize,
///         },
///         sigev_signo: TIMER_SIG,
///         sigev_notify: nc::SIGEV_SIGNAL,
///         sigev_un: nc::sigev_un_t::default(),
///     };
///     let mut timer_id = nc::timer_t::default();
///     let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, Some(&mut ev), &mut timer_id) };
///     assert!(ret.is_ok());
///     println!("timer id: {:?}", timer_id);
///
///     let flags = 0;
///     let time = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let ret = unsafe { nc::timer_settime(timer_id, flags, &time, None) };
///     assert!(ret.is_ok());
///
///     let mut cur_time = nc::itimerspec_t::default();
///     let ret = unsafe { nc::timer_gettime(timer_id, &mut cur_time) };
///     assert!(ret.is_ok());
///     println!("cur time: {:?}", cur_time);
///
///     let mask = nc::sigset_t::default();
///     let _ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///
///     let ret = unsafe { nc::timer_delete(timer_id) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn timer_gettime(timer_id: timer_t, curr: &mut itimerspec_t) -> Result<(), Errno> {
    let timer_id = timer_id as usize;
    let curr_ptr = curr as *mut itimerspec_t as usize;
    syscall2(SYS_TIMER_GETTIME, timer_id, curr_ptr).map(drop)
}

/// Arm/disarm state of per-process timer.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// fn handle_alarm(signum: i32) {
///     assert_eq!(signum, nc::SIGALRM);
/// }
///
/// fn main() {
///     const TIMER_SIG: i32 = nc::SIGRTMAX;
///
///     let sa = nc::sigaction_t {
///         sa_flags: nc::SA_SIGINFO,
///         sa_handler: handle_alarm as nc::sighandler_t,
///         ..nc::sigaction_t::default()
///     };
///     let mut old_sa = nc::sigaction_t::default();
///     let ret = unsafe { nc::rt_sigaction(TIMER_SIG, &sa, &mut old_sa, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
///
///     let tid = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let mut ev = nc::sigevent_t {
///         sigev_value: nc::sigval_t {
///             sival_ptr: &tid as *const nc::itimerspec_t as usize,
///         },
///         sigev_signo: TIMER_SIG,
///         sigev_notify: nc::SIGEV_SIGNAL,
///         sigev_un: nc::sigev_un_t::default(),
///     };
///     let mut timer_id = nc::timer_t::default();
///     let ret = unsafe { nc::timer_create(nc::CLOCK_MONOTONIC, Some(&mut ev), &mut timer_id) };
///     assert!(ret.is_ok());
///     println!("timer id: {:?}", timer_id);
///
///     let flags = 0;
///     let time = nc::itimerspec_t {
///         it_interval: nc::timespec_t::default(),
///         it_value: nc::timespec_t {
///             tv_sec: 1,
///             tv_nsec: 0,
///         },
///     };
///     let ret = unsafe { nc::timer_settime(timer_id, flags, &time, None) };
///     assert!(ret.is_ok());
///
///     let mut cur_time = nc::itimerspec_t::default();
///     let ret = unsafe { nc::timer_gettime(timer_id, &mut cur_time) };
///     assert!(ret.is_ok());
///     println!("cur time: {:?}", cur_time);
///
///     let mask = nc::sigset_t::default();
///     let _ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///
///     let ret = unsafe { nc::timer_delete(timer_id) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn timer_settime(
    timer_id: timer_t,
    flags: i32,
    new_value: &itimerspec_t,
    old_value: Option<&mut itimerspec_t>,
) -> Result<(), Errno> {
    let timer_id = timer_id as usize;
    let flags = flags as usize;
    let new_value_ptr = new_value as *const itimerspec_t as usize;
    let old_value_ptr = old_value.map_or(0, |old_value| old_value as *mut itimerspec_t as usize);
    syscall4(
        SYS_TIMER_SETTIME,
        timer_id,
        flags,
        new_value_ptr,
        old_value_ptr,
    )
    .map(drop)
}

/// Get process times.
///
/// # Example
///
/// ```
/// let mut tms = nc::tms_t::default();
/// let ret = unsafe { nc::times(&mut tms) };
/// assert!(ret.is_ok());
/// let clock = ret.unwrap();
/// assert!(clock > 0);
/// ```
pub unsafe fn times(buf: &mut tms_t) -> Result<clock_t, Errno> {
    let buf_ptr = buf as *mut tms_t as usize;
    syscall1(SYS_TIMES, buf_ptr).map(|ret| ret as clock_t)
}

/// Send a signal to a thread (obsolete).
///
/// # Example
///
/// ```
/// use core::mem::size_of;
///
/// let pid = unsafe { nc::fork() };
/// assert!(pid.is_ok());
/// let pid = pid.unwrap();
/// assert!(pid >= 0);
///
/// if pid == 0 {
///     // child process.
///     let mask = nc::sigset_t::default();
///     let ret = unsafe { nc::rt_sigsuspend(&mask, size_of::<nc::sigset_t>()) };
///     assert!(ret.is_ok());
/// } else {
///     // parent process.
///     let t = nc::timespec_t {
///         tv_sec: 1,
///         tv_nsec: 0,
///     };
///     let ret = unsafe { nc::nanosleep(&t, None) };
///     assert!(ret.is_ok());
///
///     let ret = unsafe { nc::tkill(pid, nc::SIGTERM) };
///     assert!(ret.is_ok());
/// }
/// ```
pub unsafe fn tkill(tid: i32, sig: i32) -> Result<(), Errno> {
    let tid = tid as usize;
    let sig = sig as usize;
    syscall2(SYS_TKILL, tid, sig).map(drop)
}

/// Truncate a file to a specified length.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-truncate";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::truncate(path, 64 * 1024) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn truncate<P: AsRef<Path>>(filename: P, length: off_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let length = length as usize;
    syscall2(SYS_TRUNCATE, filename_ptr, length).map(drop)
}

/// Truncate a file to a specific length.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-truncate64";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::truncate64(path, 64 * 1024) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn truncate64<P: AsRef<Path>>(path: P, len: loff_t) -> Result<(), Errno> {
    let path = PathBuf::new(path);
    let path_ptr = path.as_ptr() as usize;
    let len = len as usize;
    syscall2(SYS_TRUNCATE64, path_ptr, len).map(drop)
}

/// Set file mode creation mask.
///
/// # Example
///
/// ```
/// let new_mask = 0o077;
/// let ret = unsafe { nc::umask(new_mask) };
/// assert!(ret.is_ok());
/// let old_mask = ret.unwrap();
/// let ret = unsafe { nc::umask(old_mask) };
/// assert_eq!(ret, Ok(new_mask));
/// ```
pub unsafe fn umask(mode: mode_t) -> Result<mode_t, Errno> {
    let mode = mode as usize;
    syscall1(SYS_UMASK, mode).map(|ret| ret as mode_t)
}

/// Umount filesystem.
///
/// # Example
///
/// ```
/// let target_dir = "/tmp/nc-umount";
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
/// let ret = unsafe { nc::umount(target_dir) };
/// assert!(ret.is_err());
///
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, target_dir, nc::AT_REMOVEDIR) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn umount<P: AsRef<Path>>(name: P) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    syscall1(SYS_UMOUNT, name_ptr).map(drop)
}

/// Umount filesystem.
///
/// # Example
///
/// ```
/// let target_dir = "/tmp/nc-umount2";
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
/// let flags = 0;
/// let ret = unsafe { nc::umount2(target_dir, flags) };
/// assert!(ret.is_err());
/// assert_eq!(ret, Err(nc::EPERM));
///
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, target_dir, nc::AT_REMOVEDIR) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn umount2<P: AsRef<Path>>(name: P, flags: i32) -> Result<(), Errno> {
    let name = PathBuf::new(name);
    let name_ptr = name.as_ptr() as usize;
    let flags = flags as usize;
    syscall2(SYS_UMOUNT2, name_ptr, flags).map(drop)
}

/// Get name and information about current kernel.
///
/// # Example
///
/// ```
/// let mut buf = nc::utsname_t::default();
/// let ret = unsafe { nc::uname(&mut buf) };
/// assert!(ret.is_ok());
/// assert!(!buf.sysname.is_empty());
/// assert!(!buf.machine.is_empty());
/// ```
pub unsafe fn uname(buf: &mut utsname_t) -> Result<(), Errno> {
    let buf_ptr = buf as *mut utsname_t as usize;
    syscall1(SYS_UNAME, buf_ptr).map(drop)
}

/// Delete a name and possibly the file it refers to.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-unlink";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };;
/// assert!(ret.is_ok());
/// ```
pub unsafe fn unlink<P: AsRef<Path>>(filename: P) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    syscall1(SYS_UNLINK, filename_ptr).map(drop)
}

/// Delete a name and possibly the file it refers to.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-unlinkat";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// // /tmp folder is not empty, so this call always returns error.
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, nc::AT_REMOVEDIR) };
/// assert!(ret.is_err());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn unlinkat<P: AsRef<Path>>(dfd: i32, filename: P, flag: i32) -> Result<(), Errno> {
    let dfd = dfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let flag = flag as usize;
    syscall3(SYS_UNLINKAT, dfd, filename_ptr, flag).map(drop)
}

/// Disassociate parts of the process execution context
pub unsafe fn unshare(flags: i32) -> Result<(), Errno> {
    let flags = flags as usize;
    syscall1(SYS_UNSHARE, flags).map(drop)
}

/// Load shared library.
pub unsafe fn uselib<P: AsRef<Path>>(library: P) -> Result<(), Errno> {
    let library = PathBuf::new(library);
    let library_ptr = library.as_ptr() as usize;
    syscall1(SYS_USELIB, library_ptr).map(drop)
}

/// Create a file descriptor to handle page faults in user space.
pub unsafe fn userfaultfd(flags: i32) -> Result<i32, Errno> {
    let flags = flags as usize;
    syscall1(SYS_USERFAULTFD, flags).map(|ret| ret as i32)
}

/// Get filesystem statistics
pub unsafe fn ustat(dev: dev_t, ubuf: &mut ustat_t) -> Result<(), Errno> {
    let dev = dev as usize;
    let ubuf_ptr = ubuf as *mut ustat_t as usize;
    syscall2(SYS_USTAT, dev, ubuf_ptr).map(drop)
}

/// Change file last access and modification time.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-utime";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let time = nc::utimbuf_t {
///     actime: 100,
///     modtime: 10,
/// };
/// let ret = unsafe { nc::utime(path, &time) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn utime<P: AsRef<Path>>(filename: P, times: &utimbuf_t) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let times_ptr = times as *const utimbuf_t as usize;
    syscall2(SYS_UTIME, filename_ptr, times_ptr).map(drop)
}

/// Change time timestamps with nanosecond precision.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-utimesat";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let times = [
///     nc::timespec_t {
///         tv_sec: 100,
///         tv_nsec: 0,
///     },
///     nc::timespec_t {
///         tv_sec: 10,
///         tv_nsec: 0,
///     },
/// ];
/// let flags = nc::AT_SYMLINK_NOFOLLOW;
/// let ret = unsafe { nc::utimensat(nc::AT_FDCWD, path, &times, flags) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn utimensat<P: AsRef<Path>>(
    dirfd: i32,
    filename: P,
    times: &[timespec_t; 2],
    flags: i32,
) -> Result<(), Errno> {
    let dirfd = dirfd as usize;
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let times_ptr = times.as_ptr() as usize;
    let flags = flags as usize;
    syscall4(SYS_UTIMENSAT, dirfd, filename_ptr, times_ptr, flags).map(drop)
}

/// Change file last access and modification time.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-utimes";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let times = [
///     nc::timeval_t {
///         tv_sec: 100,
///         tv_usec: 0,
///     },
///     nc::timeval_t {
///         tv_sec: 10,
///         tv_usec: 0,
///     },
/// ];
/// let ret = unsafe { nc::utimes(path, &times) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn utimes<P: AsRef<Path>>(filename: P, times: &[timeval_t; 2]) -> Result<(), Errno> {
    let filename = PathBuf::new(filename);
    let filename_ptr = filename.as_ptr() as usize;
    let times_ptr = times.as_ptr() as usize;
    syscall2(SYS_UTIMES, filename_ptr, times_ptr).map(drop)
}

/// Create a child process and wait until it is terminated.
pub unsafe fn vfork() -> Result<pid_t, Errno> {
    syscall0(SYS_VFORK).map(|ret| ret as pid_t)
}

/// Virtually hang up the current terminal.
pub unsafe fn vhangup() -> Result<(), Errno> {
    syscall0(SYS_VHANGUP).map(drop)
}

/// Splice user page into a pipe.
pub unsafe fn vmsplice(
    fd: i32,
    iov: &iovec_t,
    nr_segs: usize,
    flags: u32,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let iov_ptr = iov as *const iovec_t as usize;
    let flags = flags as usize;
    syscall4(SYS_VMSPLICE, fd, iov_ptr, nr_segs, flags).map(|ret| ret as ssize_t)
}

/// Wait for process to change state.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::fork() };
/// match ret {
///     Err(errno) => {
///         eprintln!("fork() error: {}", nc::strerror(errno));
///         unsafe { nc::exit(1) };
///     }
///     Ok(0) => println!("[child] pid is: {}", unsafe { nc::getpid() }),
///     Ok(pid) => {
///         let mut status = 0;
///         let mut usage = nc::rusage_t::default();
///         let ret = unsafe { nc::wait4(-1, &mut status, 0, &mut usage) };
///         assert!(ret.is_ok());
///         println!("status: {}", status);
///         let exited_pid = ret.unwrap();
///         assert_eq!(exited_pid, pid);
///     }
/// }
/// ```
pub unsafe fn wait4(
    pid: pid_t,
    wstatus: &mut i32,
    options: i32,
    rusage: &mut rusage_t,
) -> Result<pid_t, Errno> {
    let pid = pid as usize;
    let wstatus_ptr = wstatus as *mut i32 as usize;
    let options = options as usize;
    let rusage_ptr = rusage as *mut rusage_t as usize;
    syscall4(SYS_WAIT4, pid, wstatus_ptr, options, rusage_ptr).map(|ret| ret as pid_t)
}

/// Wait for process to change state.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::fork() };
/// match ret {
///     Err(errno) => {
///         eprintln!("fork() error: {}", nc::strerror(errno));
///         unsafe { nc::exit(1) };
///     }
///     Ok(0) => println!("[child] pid is: {}", unsafe { nc::getpid() }),
///     Ok(pid) => {
///         let mut info = nc::siginfo_t::default();
///         let options = nc::WEXITED;
///         let mut usage = nc::rusage_t::default();
///         let ret = unsafe { nc::waitid(nc::P_ALL, -1, &mut info, options, &mut usage) };
///         match ret {
///             Err(errno) => eprintln!("waitid() error: {}", nc::strerror(errno)),
///             Ok(()) => {
///                 let exited_pid = unsafe { info.siginfo.sifields.sigchld.pid };
///                 assert_eq!(pid, exited_pid);
///             }
///         }
///     }
/// }
/// ```
pub unsafe fn waitid(
    which: i32,
    pid: pid_t,
    info: &mut siginfo_t,
    options: i32,
    ru: &mut rusage_t,
) -> Result<(), Errno> {
    let which = which as usize;
    let pid = pid as usize;
    let info_ptr = info as *mut siginfo_t as usize;
    let options = options as usize;
    let ru_ptr = ru as *mut rusage_t as usize;
    syscall5(SYS_WAITID, which, pid, info_ptr, options, ru_ptr).map(drop)
}

/// Wait for process to change state.
///
/// # Example
///
/// ```
/// let ret = unsafe { nc::fork() };
/// match ret {
///     Err(errno) => {
///         eprintln!("fork() error: {}", nc::strerror(errno));
///         unsafe { nc::exit(1) };
///     }
///     Ok(0) => println!("[child] pid is: {}", unsafe { nc::getpid() }),
///     Ok(pid) => {
///         let mut status = 0;
///         let ret = unsafe { nc::waitpid(pid, &mut status, 0) };
///         assert!(ret.is_ok());
///         let exited_pid = ret.unwrap();
///         assert_eq!(exited_pid, pid);
///     }
/// }
/// ```
pub unsafe fn waitpid(pid: pid_t, status: &mut i32, options: i32) -> Result<pid_t, Errno> {
    let pid = pid as usize;
    let status_ptr = status as *mut i32 as usize;
    let options = options as usize;
    syscall3(SYS_WAITPID, pid, status_ptr, options).map(|ret| ret as pid_t)
}

/// Write to a file descriptor.
///
/// # Example
///
/// ```
/// let path = "/tmp/nc-write";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_CREAT | nc::O_WRONLY, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let msg = "Hello, Rust!";
/// let ret = unsafe { nc::write(fd, msg.as_ptr() as usize, msg.len()) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(msg.len() as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn write(fd: i32, buf_ptr: usize, count: size_t) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    syscall3(SYS_WRITE, fd, buf_ptr, count).map(|ret| ret as ssize_t)
}

/// Write to a file descriptor from multiple buffers.
///
/// # Example
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let ret = unsafe { nc::readv(fd, &mut iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
///
/// let path_out = "/tmp/nc-writev";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path_out, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let ret = unsafe { nc::writev(fd, &iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path_out, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn writev(fd: i32, iov: &[iovec_t]) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let iov_ptr = iov.as_ptr() as usize;
    let len = iov.len();
    syscall3(SYS_WRITEV, fd, iov_ptr, len).map(|ret| ret as ssize_t)
}

/// Reposition read/write file offset.
pub unsafe fn _llseek(
    fd: i32,
    offset_high: usize,
    offset_low: usize,
    result: &mut loff_t,
    whence: i32,
) -> Result<(), Errno> {
    let fd = fd as usize;
    let result_ptr = result as *mut loff_t as usize;
    let whence = whence as usize;
    syscall5(SYS__LLSEEK, fd, offset_high, offset_low, result_ptr, whence).map(drop)
}

/// Read/write system parameters.
pub unsafe fn _sysctl(args: &mut sysctl_args_t) -> Result<(), Errno> {
    let args_ptr = args as *mut sysctl_args_t as usize;
    syscall1(SYS__SYSCTL, args_ptr).map(drop)
}

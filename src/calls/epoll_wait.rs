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

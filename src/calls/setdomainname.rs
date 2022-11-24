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

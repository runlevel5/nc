/// Load shared library.
pub unsafe fn uselib<P: AsRef<Path>>(library: P) -> Result<(), Errno> {
    let library = PathBuf::new(library);
    let library_ptr = library.as_ptr() as usize;
    syscall1(SYS_USELIB, library_ptr).map(drop)
}

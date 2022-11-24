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

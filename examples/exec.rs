use std::ffi::CStr;

fn main() {
    let cmd = "/usr/bin/echo";
    let args = [
        CStr::from_bytes_with_nul(b"/usr/bin/echo\0").unwrap(),
        CStr::from_bytes_with_nul(b"hello\0").unwrap(),
    ];
    let env = [CStr::from_bytes_with_nul(b"LANG=en_US.UTF-8\0").unwrap()];
    let ret = unsafe { nc::execve(cmd, &args, &env) };
    println!("ret: {:?}", ret);
    assert!(ret.is_ok());
}

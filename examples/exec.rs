fn main() {
    let cmd = "/usr/bin/ls";
    let args = ["/usr/bin/ls", "-l", "/"];
    let env = ["LANG=en_US.UTF-8"];
    let ret = unsafe { nc::execve(cmd, &args, &env) };
    println!("ret: {:?}", ret);
    assert!(ret.is_ok());
}

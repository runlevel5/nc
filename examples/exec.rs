fn main() {
    let cmd = "/usr/bin/ls";
    let args = ["/usr/bin/ls", "-l", "/"];
    let mut env = vec!["LANG=en_US.UTF-8".to_string()];
    for (key, value) in std::env::vars() {
        env.push(format!("{}={}", key, value));
    }
    let env_ptr: Vec<&str> = env.iter().map(|item| item.as_str()).collect();
    let ret = unsafe { nc::execve(cmd, &args, &env_ptr) };
    println!("ret: {:?}", ret);
    assert!(ret.is_ok());
}

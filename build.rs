// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn build_syscalls() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;
    let syscall_file = format!("src/syscalls/syscall_{}.c", target_arch);
    let obj_file = format!("{}/syscall.o", out_dir);

    if target.contains("i686") || target.contains("i586") {
        cmd.args.push("-m32".into());
    } else if target == "x86_64-unknown-linux-gnux32" {
        cmd.args.push("-mx32".into());
    } else if target.contains("x86_64") || target.contains("powerpc64") {
        cmd.args.push("-m64".into());
    }
    let s = Command::new("cc")
        .args(&[&syscall_file, "-m32", "-c", "-fPIC", "-o", &obj_file])
        .status()?;
    if !s.success() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to run cc command",
        )));
    }

    let s = Command::new("ar")
        .args(&["crs", "libsyscall.a", "syscall.o"])
        .current_dir(&Path::new(&out_dir))
        .status()?;
    if !s.success() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to run ar command",
        )));
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=syscall");
    Ok(())
}

fn get_page_size() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let output = Command::new("getconf").args(["PAGE_SIZE"]).output()?;
    let page_size_val = std::str::from_utf8(&output.stdout)?;
    let page_size_val = page_size_val.trim();
    let dest_path = Path::new(&out_dir).join("page_size.rs");
    let _ = fs::write(
        dest_path,
        format!("pub const PAGE_SIZE: usize = {};", page_size_val),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let rustc_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap_or_else(|_| "stable".to_string());
    if rustc_toolchain.starts_with("nightly") {
        println!("cargo:rustc-cfg=has_asm");
    } else {
        build_syscalls()?;
    }
    get_page_size()
}

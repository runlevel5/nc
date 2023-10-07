// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn guest_cc_flags(target: &str) -> Vec<&'static str> {
    println!("target: {target}");
    let mut args = Vec::new();

    if target.contains("i686") || target.contains("i586") {
        args.push("-m32");
    } else if target == "x86_64-unknown-linux-gnux32" {
        args.push("-mx32");
    } else if target.contains("x86_64") || target.contains("powerpc64") {
        // x86_64-unknown-linux-gnu
        // powerpc64-unknown-linux-gnu
        // powerpc64le-unknown-linux-gnu
        args.push("-m64");
    } else if (target.starts_with("armv7") || target.starts_with("thumbv7"))
        && (target.contains("-linux-") || target.contains("-kmc-solid_"))
    {
        // armv7 targets get to use armv7 instructions
        args.push("-march=armv7-a");

        if target.ends_with("eabihf") {
            // lowest common denominator FPU
            args.push("-mfpu=vfpv3-d16");
        }
    } else if target.contains("neon") {
        args.push("-mfpu=neon-vfpv4");
    } else if target.starts_with("arm-unknown-linux-") {
        // For us arm == armv6 by default
        args.push("-march=armv6");
        //cmd.arg("-marm");
        if target.ends_with("hf") {
            args.push("-mfpu=vfp");
        } else {
            args.push("-mfloat-abi=soft");
        }
    }

    args
}

fn build_syscalls() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let target = env::var("TARGET")?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")?;
    let syscall_file = format!("src/syscalls/syscall_{}.c", target_arch);
    let obj_file = format!("{}/syscall.o", out_dir);

    let mut cmd = Command::new("cc");
    let flags = guest_cc_flags(&target);
    cmd.args(&[&syscall_file, "-c", "-fPIC", "-o", &obj_file])
        .args(&flags);
    let s = cmd.status()?;
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

#!/usr/bin/env python3
# Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by Apache-2.0 License that can be found
# in the LICENSE file.

import os
import re
import subprocess
import sys

DEFINES = {
    "aarch64": {
        "compiler": "aarch64-linux-gnu-gcc",
        "deb": ["linux-libc-dev-arm64-cross", "gcc-aarch64-linux-gnu"],
        "include": "/usr/aarch64-linux-gnu/include",
        "errno": [
            "/usr/aarch64-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/aarch64-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/aarch64-linux-gnu/include/asm/unistd.h",
    },
    "arm": {
        "compiler": "arm-linux-gnueabihf-gcc",
        "deb": ["linux-libc-dev-armhf-cross", "gcc-arm-linux-gnueabihf"],
        "include": "/usr/arm-linux-gnueabihf/include",
        "errno": [
            "/usr/arm-linux-gnueabihf/include/asm-generic/errno-base.h",
            "/usr/arm-linux-gnueabihf/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/arm-linux-gnueabihf/include/asm/unistd.h",
    },
    # debian sid does not contain gcc mips version in multiarch
    "mips": {
        "compiler": "gcc",
        "deb": ["linux-libc-dev-mips-cross", "gcc"],
        "include": "/usr/mips-linux-gnu/include",
        "errno": [
            "/usr/mips-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/mips-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/mips-linux-gnu/include/asm/unistd.h",
        "defines": "-D_MIPS_SIM=_MIPS_SIM_ABI32",
    },
    "mips64": {
        "compiler": "mips64-linux-gnuabi64-gcc",
        "deb": ["linux-libc-dev-mips64-cross", "gcc-mips64-linux-gnuabi64"],
        "include": "/usr/mips64-linux-gnuabi64/include",
        "errno": [
            "/usr/mips64-linux-gnuabi64/include/asm-generic/errno-base.h",
            "/usr/mips64-linux-gnuabi64/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/mips64-linux-gnuabi64/include/asm/unistd.h",
    },
    "ppc64": {
        "compiler": "powerpc64-linux-gnu-gcc",
        "deb": ["linux-libc-dev-ppc64-cross", "gcc-powerpc64-linux-gnu"],
        "include": "/usr/powerpc64-linux-gnu/include",
        "errno": [
            "/usr/powerpc64-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/powerpc64-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/powerpc64-linux-gnu/include/asm/unistd.h",
    },
    "s390x": {
        "compiler": "s390x-linux-gnu-gcc",
        "deb": ["linux-libc-dev-s390x-cross", "gcc-s390x-linux-gnu"],
        "include": "/usr/s390x-linux-gnu/include",
        "errno": [
            "/usr/s390x-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/s390x-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/s390x-linux-gnu/include/asm/unistd.h",
    },
    "x86": {
        "compiler": "i686-linux-gnu-gcc",
        "deb": ["linux-libc-dev-i386-cross", "gcc-i686-linux-gnu"],
        "include": "/usr/i686-linux-gnu/include",
        "errno": [
            "/usr/i686-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/i686-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/i686-linux-gnu/include/asm/unistd.h",
    },
    "x86_64": {
        "compiler": "gcc",
        "deb": ["linux-libc-dev-amd64-cross", "gcc"],
        "include": "/usr/x86_64-linux-gnu/include",
        "errno": [
            "/usr/x86_64-linux-gnu/include/asm-generic/errno-base.h",
            "/usr/x86_64-linux-gnu/include/asm-generic/errno.h",
        ],
        "sysno": "/usr/x86_64-linux-gnu/include/asm/unistd.h",
    },
}

def read_errno(arch_name):
    header_files = get_errno_header(arch_name)
    errors = []
    for header in header_files:
        with open(header) as fh:
            errors.extend(parse_errno(fh.read()))

    lines = ["""
    // Code generated by mksysnum_linux.py; DO NOT EDIT.

    use crate::syscalls::Errno;
    """
    ]

    for errno in errors:
        if type(errno[1]) is int:
            lines.append("/// {0}".format(errno[2]))
        lines.append("pub const {0}: Errno = {1};".format(errno[0], errno[1]))

    lines.append("""
    /// Get errno description.
    pub fn strerror(errno: Errno) -> &'static str {
        match errno {
    """)

    for errno in errors:
        if type(errno[1]) is int:
            lines.append('{0} => "{1}",'.format(errno[0], errno[2]))

    lines.append("""
        _ => "Unknown errno!",
        }
    }
    """)
    return lines 

def parse_errno(content):
    # For `define    EKEYEXPIRED     127     /* Key has expired */`
    errno_pattern = re.compile("^#define\s+(E\w+)\s+(\d+)\s+/\\*([^\\*]+)\\*/")
    # For `define   EDEADLOCK       EDEADLK`
    alias_pattern = re.compile("^#define\s+(E\w+)\s+(E\w+)")
    errors = []
    for line in content.split('\n'):
        m = errno_pattern.match(line)
        if m:
            errors.append((m.group(1), int(m.group(2)), m.group(3).strip()))
        else:
            m = alias_pattern.match(line)
            if m:
                errors.append((m.group(1), m.group(2)))
    return errors

def read_sysno(arch_name):
    compiler = get_compiler(arch_name)
    header_file = get_sysno_header(arch_name)
    include_dir = get_include_dir(arch_name)
    defines = get_defines(arch_name)
    if defines:
        cmd = [compiler, "-I", include_dir, "-E", "-dD", defines, header_file]
    else:
        cmd = [compiler, "-I", include_dir, "-E", "-dD", header_file]
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE)
    out, err = p.communicate()
    if p.returncode != 0 or err:
        print(err)
        sys.exit(1)
    return parse_sysno(out.decode())

def parse_sysno(content):
    def f(name, num):
        num = int(num)
        # Ignore deprecated syscalls
        if num > 999:
            return

        nonlocal offset
        num = offset + num
        line = "pub const SYS_{0}: Sysno = {1};".format(name.upper(), num)
        nonlocal lines
        lines.append(line)

    lines = [
        "",
        "// Code generated by mksysnum_linux.py; DO NOT EDIT.",
        "",
        "use crate::syscalls::Sysno;",
        "",
    ]

    pattern0 = re.compile("^#define __NR_Linux\s+([0-9]+)")
    pattern1 = re.compile("^#define __NR_(\w+)\s+(\d+)")
    pattern2 = re.compile("^#define __NR_(\w+)\s+\(__NR_Linux \+ ([0-9]+)")
    pattern3 = re.compile("^#define __NR3264_(\w+)\s+([0-9]+)")
    pattern4 = re.compile("^#define __NR_(\w+)\s+\(\w+\s+\+\s+([0-9]+)\)")
    prev = 0
    offset = 0

    for line in content.split("\n"):
        # Ignore syscall
        if line.startswith("#define __NR_syscalls") or "_Linux_syscalls" in line:
            continue

        m0 = pattern0.match(line)
        if m0:
            # For mips/mips64, extract offset.
            offset = int(m0.group(1))
            continue

        m1 = pattern1.match(line)
        if m1:
            f(m1.group(1), m1.group(2))
            continue

        m2 = pattern2.match(line)
        if m2:
            f(m2.group(1), m2.group(2))
            continue

        m3 = pattern3.match(line)
        if m3:
            prev = int(m3.group(2))
            f(m3.group(1), m3.group(2))
            continue

        m4 = pattern4.match(line)
        if m4:
            num = prev + int(m4.group(2))
            f(m4.group(1), str(num))
            continue

    return lines

def get_compiler(arch_name):
    return DEFINES[arch_name]["compiler"]

def get_deb(arch_name):
    return DEFINES[arch_name]["deb"]

def get_include_dir(arch_name):
    return DEFINES[arch_name]["include"]

def get_errno_header(arch_name):
    return DEFINES[arch_name]["errno"]

def get_sysno_header(arch_name):
    return DEFINES[arch_name]["sysno"]

def get_defines(arch_name):
    return DEFINES[arch_name].get("defines", "")

def get_arch_names():
    return DEFINES.keys()

def rust_fmt(filename):
    subprocess.run(["rustfmt", filename])

def gen_errno_and_sysno(os_name, arch_name):
    folder_name = "{0}-{1}".format(os_name, arch_name)
    platform_folder = os.path.join("platform", folder_name)
    #os.makedirs(platform_folder, exist_ok=True)

    errno_lines = read_errno(arch_name)
    errno_content = "\n".join(errno_lines)
    errno_file = os.path.join(platform_folder, "errno.rs")
    with open(errno_file, "w") as fh:
        fh.write(errno_content)
    rust_fmt(errno_file)

    sysno_lines = read_sysno(arch_name)
    sysno_content = "\n".join(sysno_lines)
    sysno_file = os.path.join(platform_folder, "sysno.rs")
    with open(sysno_file, "w") as fh:
        fh.write(sysno_content)
    rust_fmt(sysno_file)

def install_deb(deb_list):
    print("install_deb:", deb_list)
    cmd = ["apt", "install", "-y"]
    cmd.extend(deb_list)
    cmd = " ".join(cmd)
    subprocess.run(cmd, shell=True)

def main():
    if len(sys.argv) > 3 or len(sys.argv) < 2:
        print("Usage: %s arch" % sys.argv[0])
        sys.exit(1)
    if sys.argv[1] == "-e":
        with open(sys.argv[2]) as fh:
            content = fh.read()
            print("\n".join(parse_errno(content)))
        return
    elif sys.argv[1] == "-s":
        with open(sys.argv[2]) as fh:
            content = fh.read()
            print("\n".join(parse_sysno(content)))
        return

    arch_name = sys.argv[1]
    os_name = "linux"
    if arch_name == "all":
        for arch_name in get_arch_names():
            try:
                gen_errno_and_sysno(os_name, arch_name)
            except FileNotFoundError:
                install_deb(get_deb(arch_name))
    else:
        gen_errno_and_sysno(os_name, arch_name)

if __name__ == "__main__":
    main()

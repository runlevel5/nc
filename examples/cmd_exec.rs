// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::process::Command;

fn main() {
    let mut cmd = Command::new("/usr/bin/ls");
    cmd.args(["-l", "/"]);
    let child = cmd.spawn();
    println!("child: {:?}", child);
}
use std::{env, process::Command, str};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let minor = match rustc_minor_version() {
        Some(x) => x,
        None => return,
    };

    if minor >= 36 {
        println!("cargo:rustc-cfg=stable_1_36");
    }
}

fn rustc_minor_version() -> Option<u32> {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| {
            str::from_utf8(&output.stdout).ok().and_then(|version| {
                let mut pieces = version.split('.');
                if pieces.next() != Some("rustc 1") {
                    return None;
                }
                pieces.next()?.parse().ok()
            })
        })
}

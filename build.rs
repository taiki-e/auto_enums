// rustc-cfg emitted by the build script are *not* public API.

#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{env, process::Command, str};

fn main() {
    let minor = match rustc_version() {
        Some(version) => version,
        None => {
            println!("cargo:warning={}: unable to determine rustc version", env!("CARGO_PKG_NAME"));
            return;
        }
    };

    // Note that this is `<` (less than), not `>=` (greater than or equal).
    // This allows treating as the latest stable rustc is used when the build
    // script doesn't run. This is useful for non-cargo build systems that don't
    // run the build script.
    if minor < 36 {
        println!("cargo:rustc-cfg=stable_lt_1_36");
    }
}

fn rustc_version() -> Option<u32> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).args(&["--version", "--verbose"]).output().ok()?;
    let mut release = str::from_utf8(&output.stdout)
        .ok()?
        .lines()
        .find(|line| line.starts_with("release: "))
        .map(|line| &line["release: ".len()..])?
        .splitn(2, '-');
    let version = release.next().unwrap();
    let _channel = release.next().unwrap_or_default();
    let mut digits = version.splitn(3, '.');
    let major = digits.next()?.parse::<u32>().ok()?;
    if major != 1 {
        return None;
    }
    let minor = digits.next()?.parse::<u32>().ok()?;
    let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
    Some(minor)
}

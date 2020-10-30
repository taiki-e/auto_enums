#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

use autocfg::AutoCfg;

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    let cfg = match AutoCfg::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!(
                "cargo:warning={}: unable to determine rustc version: {}",
                env!("CARGO_PKG_NAME"),
                e
            );
            return;
        }
    };

    if cfg.probe_rustc_version(1, 36) {
        println!("cargo:rustc-cfg=stable_1_36");
    }
}

# Unreleased

* Make `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write` optional

# 0.1.3 - 2018-12-15

* Change `#[enum_derive(Error)]` implementation
  In Rust 1.33, `Error::cause` is deprecated. In the new implementation, `Error::cause` is optional for Rust 1.33 and later. In versions less than 1.33, `Error::cause` is always implemented.

# 0.1.2 - 2018-12-15

* Move features of derive/utils to [derive_utils](https://github.com/taiki-e/derive_utils)
* Align version number of `auto_enumerate` and `auto_enums`.

# 0.1.1 - 2018-12-13

* Rename from `auto_enumerate` to `auto_enums`.

# 0.1.0 - 2018-12-09

Initial release

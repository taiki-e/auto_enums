# Unreleased

* Add support for `break` in loop. This includes support for nested loops and labeled `break`.
* Update minimum derive_utils version to 0.5.0

# 0.2.1 - 2018-12-22

* Update minimum derive_utils version to 0.4.0

# 0.2.0 - 2018-12-20

* Make `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write` optional
* Support `return` in function and closure

# 0.1.3 - 2018-12-15

* Change `#[enum_derive(Error)]` implementation<br>
  In Rust 1.33, `Error::cause` is deprecated. In the new implementation, `Error::cause` is optional for Rust 1.33 and later. In versions less than 1.33, `Error::cause` is always implemented.

# 0.1.2 - 2018-12-15

* Move features of derive/utils to [derive_utils](https://github.com/taiki-e/derive_utils)
* Align version number of `auto_enumerate` and `auto_enums`.

# 0.1.1 - 2018-12-13

* Rename from `auto_enumerate` to `auto_enums`.

# 0.1.0 - 2018-12-09

Initial release

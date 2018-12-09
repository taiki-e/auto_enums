// [std|core]::iter
pub(crate) mod iter;

// [std|core]::ops
pub(crate) mod ops;

// [std|core]::convert
pub(crate) mod convert;

// [std|core]::fmt
pub(crate) mod fmt;

// [std|core]::future
pub(crate) mod future;

// std::io
#[cfg(feature = "std")]
pub(crate) mod io;

// std::error
#[cfg(feature = "std")]
pub(crate) mod error;

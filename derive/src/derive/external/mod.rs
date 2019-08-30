#[cfg(feature = "futures")]
pub(crate) mod futures;

#[cfg(feature = "futures01")]
pub(crate) mod futures01;

#[cfg(feature = "rayon")]
pub(crate) mod rayon;

#[cfg(feature = "serde")]
pub(crate) mod serde;

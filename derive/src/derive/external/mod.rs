// https://docs.rs/futures/0.1
#[cfg(feature = "futures01")]
pub(crate) mod futures01;
// https://docs.rs/futures/0.3
#[cfg(feature = "futures03")]
pub(crate) mod futures03;
// https://docs.rs/rayon/1
#[cfg(feature = "rayon")]
pub(crate) mod rayon;
// https://docs.rs/serde/1
#[cfg(feature = "serde")]
pub(crate) mod serde;
// https://docs.rs/tokio/0.1
#[cfg(feature = "tokio01")]
pub(crate) mod tokio01;
// https://docs.rs/tokio/0.2
#[cfg(feature = "tokio02")]
pub(crate) mod tokio02;
// https://docs.rs/tokio/0.3
#[cfg(feature = "tokio03")]
pub(crate) mod tokio03;

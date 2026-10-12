pub mod local;

#[cfg(feature = "kugou")]
pub mod kugou;

#[cfg(feature = "netease")]
pub mod netease;

pub use local::LocalProvider;

#[cfg(feature = "kugou")]
pub use kugou::KugouProvider;

#[cfg(feature = "netease")]
pub use netease::NeteaseProvider;

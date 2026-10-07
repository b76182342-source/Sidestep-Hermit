pub mod local;

#[cfg(feature = "kugou")]
pub mod kugou;

pub use local::LocalProvider;

#[cfg(feature = "kugou")]
pub use kugou::KugouProvider;

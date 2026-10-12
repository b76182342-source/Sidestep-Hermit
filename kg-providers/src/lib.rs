pub mod local;

#[cfg(feature = "kugou")]
pub mod kugou;

#[cfg(feature = "netease")]
pub mod netease;

#[cfg(feature = "qqmusic")]
pub mod qqmusic;

pub use local::LocalProvider;

#[cfg(feature = "kugou")]
pub use kugou::KugouProvider;

#[cfg(feature = "netease")]
pub use netease::NeteaseProvider;

#[cfg(feature = "qqmusic")]
pub use qqmusic::QQMusicProvider;

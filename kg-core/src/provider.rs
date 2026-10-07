use async_trait::async_trait;

use crate::{
    error::Result, Lyrics, Page, Paged, SearchItem, SearchKind, StreamSource, Track, TrackId,
};

/// 音乐提供方抽象
#[async_trait]
pub trait MusicProvider: Send + Sync {
    /// 提供方唯一标识
    fn id(&self) -> &'static str;

    /// 提供方显示名称
    fn name(&self) -> &'static str;

    /// 搜索
    async fn search(&self, query: &str, kind: SearchKind, page: Page) -> Result<Paged<SearchItem>>;

    /// 获取音轨详情
    async fn track(&self, id: &TrackId) -> Result<Track>;

    /// 获取流媒体来源 (仅返回合法授权的来源)
    async fn stream_source(&self, id: &TrackId) -> Result<StreamSource>;

    /// 获取歌词
    async fn lyrics(&self, id: &TrackId) -> Result<Option<Lyrics>>;
}

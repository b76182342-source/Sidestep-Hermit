use std::time::Duration;

use super::lrc::LrcLine;

/// 歌词同步器
pub struct LyricsSynchronizer {
    lines: Vec<LrcLine>,
    current_index: Option<usize>,
}

impl LyricsSynchronizer {
    pub fn new(lrc_content: &str) -> Result<Self, String> {
        use super::lrc::LrcParser;
        let (_, lines) = LrcParser::parse(lrc_content)?;
        Ok(Self {
            lines,
            current_index: None,
        })
    }

    /// 更新播放位置，返回当前应该显示的歌词行索引
    pub fn update(&mut self, position: Duration) -> Option<usize> {
        if self.lines.is_empty() {
            return None;
        }

        // 二分查找最接近的行
        let mut left = 0;
        let mut right = self.lines.len();
        let mut result = None;

        while left < right {
            let mid = (left + right) / 2;
            if self.lines[mid].timestamp <= position {
                result = Some(mid);
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        self.current_index = result;
        result
    }

    /// 获取当前行
    pub fn current_line(&self) -> Option<&LrcLine> {
        self.current_index.and_then(|idx| self.lines.get(idx))
    }

    /// 获取所有歌词行（用于显示完整列表）
    pub fn all_lines(&self) -> &[LrcLine] {
        &self.lines
    }

    /// 获取当前行索引
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// 获取上下文（当前行及前后各 N 行）
    pub fn context(&self, n: usize) -> Vec<(usize, &LrcLine)> {
        let current = match self.current_index {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        let start = current.saturating_sub(n);
        let end = (current + n + 1).min(self.lines.len());

        (start..end).map(|i| (i, &self.lines[i])).collect()
    }
}

#[cfg(test)]
mod tests {
    // 测试代码允许 unwrap/expect：断言失败就该直接 panic。
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn test_synchronizer() {
        let lrc = r#"
[ti:Test Song]
[00:00.50]First line
[00:05.00]Second line
[00:10.00]Third line
        "#;

        let mut sync = LyricsSynchronizer::new(lrc).unwrap();

        // 在第一行之前
        assert_eq!(sync.update(Duration::from_millis(300)), None);

        // 第一行
        assert_eq!(sync.update(Duration::from_millis(500)), Some(0));
        assert_eq!(sync.current_line().unwrap().text, "First line");

        // 第二行
        assert_eq!(sync.update(Duration::from_millis(5000)), Some(1));
        assert_eq!(sync.current_line().unwrap().text, "Second line");

        // 第三行
        assert_eq!(sync.update(Duration::from_millis(10000)), Some(2));
        assert_eq!(sync.current_line().unwrap().text, "Third line");
    }

    #[test]
    fn test_context() {
        let lrc = r#"
[00:00.00]Line 0
[00:01.00]Line 1
[00:02.00]Line 2
[00:03.00]Line 3
[00:04.00]Line 4
        "#;

        let mut sync = LyricsSynchronizer::new(lrc).unwrap();
        sync.update(Duration::from_millis(2000));

        let context = sync.context(1);
        assert_eq!(context.len(), 3);
        assert_eq!(context[0].1.text, "Line 1");
        assert_eq!(context[1].1.text, "Line 2");
        assert_eq!(context[2].1.text, "Line 3");
    }
}

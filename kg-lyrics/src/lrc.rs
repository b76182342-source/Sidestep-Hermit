use nom::{
    bytes::complete::{take_until, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct LrcLine {
    pub timestamp: Duration,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct LrcMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub by: Option<String>,
    pub offset: i64,
}

pub struct LrcParser;

impl LrcParser {
    /// 解析 LRC 格式歌词
    pub fn parse(content: &str) -> Result<(LrcMetadata, Vec<LrcLine>), String> {
        let mut metadata = LrcMetadata::default();
        let mut lines = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // 歌词行必须先于元数据尝试：`[00:12.34]` 也能被元数据规则匹配成
            // key="00" / value="12.34"，顺序反了会把整首歌的歌词全部吞掉。
            if let Ok((_, parsed_lines)) = parse_lyric_line(line) {
                if !parsed_lines.is_empty() {
                    lines.extend(parsed_lines);
                    continue;
                }
            }

            // 再尝试解析元数据
            if let Ok((_, (key, value))) = parse_metadata(line) {
                match key {
                    "ti" => metadata.title = Some(value.to_string()),
                    "ar" => metadata.artist = Some(value.to_string()),
                    "al" => metadata.album = Some(value.to_string()),
                    "by" => metadata.by = Some(value.to_string()),
                    "offset" => {
                        if let Ok(offset) = value.parse::<i64>() {
                            metadata.offset = offset;
                        }
                    }
                    _ => {}
                }
            }
        }

        // 应用偏移量
        if metadata.offset != 0 {
            for line in &mut lines {
                let millis = line.timestamp.as_millis() as i64 + metadata.offset;
                line.timestamp = Duration::from_millis(millis.max(0) as u64);
            }
        }

        // 按时间戳排序
        lines.sort_by_key(|l| l.timestamp);

        Ok((metadata, lines))
    }

    /// 获取当前播放位置对应的歌词行索引
    pub fn find_current_line(lines: &[LrcLine], position: Duration) -> Option<usize> {
        if lines.is_empty() {
            return None;
        }

        for (i, line) in lines.iter().enumerate().rev() {
            if position >= line.timestamp {
                return Some(i);
            }
        }

        None
    }
}

/// 解析元数据标签 [key:value]
///
/// key 必须是纯字母，这样 `[00:12.34]` 这种时间戳不会被误判为元数据。
fn parse_metadata(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        char('['),
        tuple((
            take_while1(|c: char| c.is_alphabetic()),
            preceded(char(':'), take_until("]")),
        )),
        char(']'),
    )(input)
}

/// 解析时间戳 [mm:ss.xx] 或 [mm:ss]
fn parse_timestamp(input: &str) -> IResult<&str, Duration> {
    map_res(
        delimited(
            char('['),
            tuple((
                digit1,                           // 分钟
                preceded(char(':'), digit1),      // 秒
                opt(preceded(char('.'), digit1)), // 可选的毫秒
            )),
            char(']'),
        ),
        |(min, sec, ms): (&str, &str, Option<&str>)| {
            let minutes: u64 = min.parse().map_err(|_| "Invalid minutes")?;
            let seconds: u64 = sec.parse().map_err(|_| "Invalid seconds")?;
            let millis: u64 = if let Some(ms_str) = ms {
                // 处理 .xx 或 .xxx 格式
                let ms_val: u64 = ms_str.parse().map_err(|_| "Invalid milliseconds")?;
                if ms_str.len() == 2 {
                    ms_val * 10
                } else {
                    ms_val
                }
            } else {
                0
            };

            let total_millis = minutes * 60_000 + seconds * 1_000 + millis;
            Ok::<Duration, &str>(Duration::from_millis(total_millis))
        },
    )(input)
}

/// 解析一行歌词 (可能包含多个时间戳)
fn parse_lyric_line(input: &str) -> IResult<&str, Vec<LrcLine>> {
    let (remaining, timestamps) = many0(parse_timestamp)(input)?;
    let (remaining, _) = multispace0(remaining)?;
    let text = remaining.trim().to_string();

    let lines: Vec<LrcLine> = timestamps
        .into_iter()
        .map(|timestamp| LrcLine {
            timestamp,
            text: text.clone(),
        })
        .collect();

    Ok(("", lines))
}

/// 便捷函数
pub fn parse_lrc(content: &str) -> Result<Vec<LrcLine>, String> {
    let (_, lines) = LrcParser::parse(content)?;
    Ok(lines)
}

#[cfg(test)]
mod tests {
    // 测试代码允许 unwrap/expect：断言失败就该直接 panic。
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn test_parse_timestamp() {
        let (_, ts) = parse_timestamp("[00:12.34]").unwrap();
        assert_eq!(ts, Duration::from_millis(12_340));

        let (_, ts) = parse_timestamp("[01:23.45]").unwrap();
        assert_eq!(ts, Duration::from_millis(83_450));

        let (_, ts) = parse_timestamp("[02:30]").unwrap();
        assert_eq!(ts, Duration::from_millis(150_000));
    }

    #[test]
    fn test_parse_metadata() {
        let (_, (key, val)) = parse_metadata("[ti:Song Title]").unwrap();
        assert_eq!(key, "ti");
        assert_eq!(val, "Song Title");
    }

    #[test]
    fn test_parse_lyric_line() {
        let (_, lines) = parse_lyric_line("[00:12.34]Hello World").unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "Hello World");
        assert_eq!(lines[0].timestamp, Duration::from_millis(12_340));

        // 多时间戳
        let (_, lines) = parse_lyric_line("[00:12.34][00:56.78]Chorus").unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "Chorus");
        assert_eq!(lines[1].text, "Chorus");
    }

    #[test]
    fn test_parse_full_lrc() {
        let lrc = r#"
[ti:Test Song]
[ar:Test Artist]
[al:Test Album]
[00:12.34]First line
[00:23.45]Second line
[00:34.56][00:56.78]Repeated line
        "#;

        let (metadata, lines) = LrcParser::parse(lrc).unwrap();
        assert_eq!(metadata.title, Some("Test Song".into()));
        assert_eq!(metadata.artist, Some("Test Artist".into()));
        assert_eq!(lines.len(), 4);
    }
}

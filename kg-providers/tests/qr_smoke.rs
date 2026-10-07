//! 酷狗联网冒烟测试（需要真实网络，默认忽略）。
//!
//! 运行：
//! `cargo test -p kg-providers --features kugou --test qr_smoke -- --ignored --nocapture`
#![cfg(feature = "kugou")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use kg_providers::kugou::KugouAuth;

#[tokio::test]
#[ignore = "需要访问酷狗接口"]
async fn qr_challenge_returns_scan_url() {
    let mut auth = KugouAuth::new().expect("build auth");

    let challenge = auth.request_qr().await.expect("request qr");

    println!("qr key  = {}", challenge.key);
    println!("scan url= {}", challenge.url);

    assert!(!challenge.key.is_empty(), "二维码 key 不应为空");
    assert!(
        challenge
            .url
            .starts_with("https://h5.kugou.com/apps/loginQRCode/"),
        "扫码 URL 前缀应与官方登录页一致，实际: {}",
        challenge.url
    );

    let (status, info) = auth.check_qr_status().await.expect("check qr");
    println!("首次轮询 status = {status}, 凭据 = {}", info.is_some());
    assert!(info.is_none(), "尚未扫码时不应返回凭据");
}

#[tokio::test]
#[ignore = "需要访问酷狗接口"]
async fn search_returns_playable_hits() {
    let mut auth = KugouAuth::new().expect("build auth");

    let hits = auth.search("夜航船", 1, 10).await.expect("search");

    println!("命中 {} 条", hits.len());
    for (i, h) in hits.iter().take(5).enumerate() {
        println!(
            "  {}. {} — {} 《{}》 {}s hash={}",
            i + 1,
            h.title,
            h.artist,
            h.album,
            h.duration_secs,
            h.hash
        );
    }

    assert!(!hits.is_empty(), "搜索应有结果");
    assert!(
        hits.iter().all(|h| !h.hash.is_empty()),
        "每条结果都应带可用 hash"
    );
}

#[tokio::test]
#[ignore = "需要访问酷狗接口"]
async fn daily_recommend_returns_hits() {
    let mut auth = KugouAuth::new().expect("build auth");

    let hits = auth.daily().await.expect("daily recommend");

    println!("每日推荐 {} 条", hits.len());
    for h in hits.iter().take(5) {
        println!("  {} — {} 《{}》", h.title, h.artist, h.album);
    }

    assert!(!hits.is_empty(), "每日推荐不应为空");
}

#[tokio::test]
#[ignore = "需要访问酷狗接口"]
async fn lyrics_are_timed() {
    let mut auth = KugouAuth::new().expect("build auth");

    let hits = auth.search("夜航船", 1, 5).await.expect("search");
    let first = hits.first().expect("至少一条结果");

    let lines = auth.lyrics(&first.hash).await.expect("lyrics");

    println!("《{}》歌词 {} 行", first.title, lines.len());
    for l in lines.iter().take(6) {
        println!("  [{}ms] {}", l.time_ms, l.text);
    }

    assert!(!lines.is_empty(), "应能取到歌词");
    assert!(
        lines.windows(2).all(|w| w[0].time_ms <= w[1].time_ms),
        "歌词时间轴应单调不减"
    );
}

fn main() {
    // 测试一些常见的 hash 值
    let test_hashes = vec![
        "ABC123DEF456",
        "1234567890AB",
        "FEDCBA987654",
    ];

    for hash in test_hashes {
        let hash_upper = hash.to_uppercase();
        let dir = &hash_upper[0..2];
        let url = format!(
            "https://imge.kugou.com/stdmusic/400/{}/{}.jpg",
            dir, hash_upper
        );
        println!("Hash: {} -> URL: {}", hash, url);
    }
}

use std::{thread, time::Duration};
use std::time::SystemTime;

/// 简单的“模拟出块”循环：每隔 N 秒打印一次
pub fn run(block_interval_secs: u64) -> ! {
    let mut height: u64 = 0;
    println!("🧱 Mock block production loop started (every {block_interval_secs}s) ...");
    loop {
        height += 1;
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        println!("💥 Block #{height} at {now}");
        thread::sleep(Duration::from_secs(block_interval_secs));
    }
}


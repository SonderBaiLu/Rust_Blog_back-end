use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("🚀 启动 1000 个并发任务实验...");
    let start = Instant::now();

    let mut handles = Vec::with_capacity(1000);

    // 瞬间派发 1000 个轻量级协程任务
    for i in 0..1000 {
        let handle = tokio::spawn(async move {
            // 模拟 100ms 的网络延迟或数据库查询等待
            std::thread::sleep(std::time::Duration::from_millis(100));
            i
        });
        handles.push(handle);
    }

    // 等待所有任务完成并汇总
    let mut sum: u64 = 0;
    for handle in handles {
        sum += handle.await.unwrap();
    }

    let elapsed = start.elapsed();
    println!("✅ 1000 个任务全部执行完毕！校验和: {}", sum);
    println!("⏱️ 如果串行执行理论需要: 100 秒 (1000 * 100ms)");
    println!("⚡ 真实并发耗时: {:.2?}", elapsed);
    println!("💡 提速倍率: 约 {} 倍", 100_000 / elapsed.as_millis().max(1));
}

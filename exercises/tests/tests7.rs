use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间的秒数
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 使用正确的格式输出环境变量
    // 确保输出的格式符合 Cargo 的期望：`cargo:KEY=VALUE`
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
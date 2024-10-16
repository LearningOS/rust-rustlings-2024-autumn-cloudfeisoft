use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {
    // 获取当前时间的秒数
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 使用正确的格式输出环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 检查是否设置了环境变量或特征标志来决定测试是否通过
    if env::var("TEST_FOO_PASS").is_ok() || env::var("CARGO_FEATURE_pass").is_ok() {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
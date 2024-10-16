use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间的秒数
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 设置环境变量 `TEST_FOO` 为当前时间的秒数
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 "pass" 特征以使 tests8 的测试用例提前返回
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
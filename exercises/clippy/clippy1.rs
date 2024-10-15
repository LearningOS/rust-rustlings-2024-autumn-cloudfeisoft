fn main() {
    let pi = std::f32::consts::PI; // 使用标准库中的 PI 常数
    let radius = 5.00f32; // 直接指定类型

    // 使用 f32 的 powf 方法进行幂运算
    let area = pi * radius.powf(2.0);

    println!(
        "The area of a circle with radius {:.2} is {:.5}",
        radius, area
    );
}
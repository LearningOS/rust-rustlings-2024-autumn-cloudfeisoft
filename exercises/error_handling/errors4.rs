fn main() {
    use std::thread;
    use std::time::Duration;

    fn generate_workout(intensity: u32, random_number: u32) {
        let mut i = 0; // 初始化 i
        let expensive_closure = |num: u32| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num + i
        };

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_closure(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_closure(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_closure(intensity)
                );
            }
        }
    }
}
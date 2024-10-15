fn main() {
    // Use match or if let to handle the Option without panicking
    let my_option: Option<()> = None;
    match my_option {
        None => println!("Option is none"),
        _ => println!("Option is some"),
    }

    // Remove the trailing comma in array declaration
    let my_arr = [
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Use vec! macro to initialize the vector with values
    let my_empty_vec = vec![1, 2, 3, 4, 5];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // Swap two variables using a tuple swap
    let mut value_a = 45;
    let mut value_b = 66;
    value_a = std::mem::replace(&mut value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}
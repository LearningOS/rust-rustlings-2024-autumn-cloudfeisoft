macro_rules! my_macro {
    // This pattern matches if there are no arguments provided to the macro.
    () => {
        println!("Check out my macro!");
    };
    // This pattern matches if there is exactly one argument provided to the macro.
    ($val:expr) => {
        println!("Check out my macro with an argument: {}", $val);
    };
    // This pattern matches if there are two arguments provided to the macro.
    ($val1:expr, $val2:expr) => {
        println!("Check out my macro with arguments: {} and {}", $val1, $val2);
    };
}

fn main() {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    
    fn main() {
        my_macro!(); // Invoke the macro with !
    }
}
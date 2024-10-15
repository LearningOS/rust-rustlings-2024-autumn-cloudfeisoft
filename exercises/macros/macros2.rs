// Define the macro first
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// Then define the main function
fn main() {
    // Now you can use the macro
    my_macro!();
}
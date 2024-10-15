mod macros {
    // Define the macro within the module.
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // Create a public function that invokes the macro.
    pub fn invoke_macro() {
        my_macro!();
    }
}

fn main() {
    // Call the public function from the main module to invoke the macro.
    macros::invoke_macro();
}
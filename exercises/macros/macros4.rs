// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    (7777) => {
        println!("Special case for 7777!");
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

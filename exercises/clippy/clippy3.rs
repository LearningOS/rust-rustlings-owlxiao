// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

#[allow(clippy::redundant_pattern_matching)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap_or_default();
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    assert!(my_empty_vec.is_empty());

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

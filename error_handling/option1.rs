// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)

fn main() {
    let mut list = vec![3];

    match list.pop() {
        Some(last) => println!("The last item in the list is {:?}", last),
        None => println!("empty list!")
    }

    match list.pop() {
        Some(second_to_last) => println!("The second-to-last item in the list is {:?}", second_to_last),
        None => println!("empty list!")
    }
}

























// Try using a `match` statement where the arms are `Some(thing)` and `None`.
// Or set a default value to print out if you get `None` by using the
// function `unwrap_or`.
// Or use an `if let` statement on the result of `pop()` to both destructure
// a `Some` value and only print out something if we have a value!

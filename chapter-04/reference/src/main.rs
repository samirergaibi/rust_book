// We call the action of creating a reference borrowing.
// As in real life, if a person owns something, you can borrow it from them.
// When you’re done, you have to give it back. You don’t own it.

fn main() {
    // Pass a reference to the value, ampersand (&) = reference
    let s1 = String::from("Samir");
    let len = calculate_length(&s1);
    println!("s1 = {}, len = {}", s1, len);

    // Change a borrowed value
    let mut first_name = String::from("Samir");
    change(&mut first_name);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// We can not change the reference since we don't own it.
// fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

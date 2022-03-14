fn main() {
    let age = 18;
    if age < 18 {
        println!("You are too young!");
    } else {
        println!("Welcome!");
    }

    // Return from if statements
    let condition = true;
    let value = if condition { 18 } else { 0 };
    println!("value is: {}", value);
}

fn main() {
    println!("Hello, world!");
    print_age(28, "years");

    let x = five();
    println!("five: {}", x);

    println!("Result is: {}", add(5, 5));
}

fn print_age(age: i32, unit: &str) {
    println!("The age is: {} {}", age, unit);
}

fn five() -> i32 {
    5
}

// Implicit return (Try to add semi-color to the end of the expression and see what happens)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

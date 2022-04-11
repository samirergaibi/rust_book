fn main() {
    let s = String::from("hello");
    let s2 = s;
    // s.push_str(", world");

    // println!("{}", s);
    println!("{}", s2);

    // CLONE
    let name = String::from("Samir");
    let deep_copy = name.clone();

    println!("name = {}, deep_copy = {}", name, deep_copy);

    // FUNCTIONS and OWNERSHIP
    let last_name = String::from("Ergaibi");

    print_last_name(last_name);
    // Doesnt work since the value is moved
    // println!("{}", last_name);

    // Functions that retun a value gives ownership to the function that calls it
    let received_ownership = gives_ownership();
    println!("{}", received_ownership);

    // This function takes a string and returns one back
    let middle_name = String::from("Michel");
    let still_middle_name = takes_and_gives_back(middle_name);
    println!("{}", still_middle_name);
}

fn print_last_name(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// Having to pass something back just to reuse it would be vary annoying, and it's a really common thing to do
// Check out the reference dir to see how to avoid that

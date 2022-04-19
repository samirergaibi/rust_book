fn main() {
    let middle_name = "Michel";
    let first_name = String::from("Samir");

    println!("Hello there friend! My name is {}", first_name);

    something(&first_name);
    something(middle_name);
    have_fun();
}

fn something(name: &str) {
    println!("My full name is {} Ergaibi!", name);
}

fn have_fun() {
    let sentence = String::from("Hello world");
    let hello = &sentence[..5];
    let world = &sentence[6..];

    println!("{} {}!", hello, world);
}

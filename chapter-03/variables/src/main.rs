fn main() {
    // Variables are immutable by default
    // let x = 5;
    let mut x = 5;
    println!("The value os x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("---------------------------------------");
    // Shadowing
    let shadowing = 5;

    let shadowing = shadowing + 1;
    {
        let shadowing = shadowing * 2;
        println!(
            "The value of shadowing in the inner scope is: {}",
            shadowing
        );
    }
    println!("The value of shadowing is: {}", shadowing);
}

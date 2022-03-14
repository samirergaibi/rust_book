fn main() {
    // Signed or unsigned integers
    let some_number: i8 = 28;
    println!("some_number: {}", some_number);

    let high_number: u128 = 28_000_000_000;
    println!("high_number: {}", high_number);

    // Compound Types
    //Tuples
    let tup: (i32, f64, u8) = (600, 6.4, 1);
    println!("tup: {}", tup.0);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {}", arr[1]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Number of months is: {}", months.len());
}

fn main() {
    // regular_loop();
    // while_loop();
    // loop_over_collection()
    // for_loop();
    fizz_buzz()
}

fn regular_loop() {
    loop {
        println!("again");
        break;
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("number: {}", number);
        number -= 1;
    }
    println!("LIFT OFF!");
}

fn loop_over_collection() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {}", element);
    }
}

fn fizz_buzz() {
    for number in 1..100 {
        if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }
}

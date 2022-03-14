use std::io;

fn main() {
    let convert_to_cel: bool;
    loop {
        println!("Do you want to convert Fahrenheit to Celsius? (y/n)?");
        let mut fahr_to_cel = String::new();
        io::stdin()
            .read_line(&mut fahr_to_cel)
            .expect("Could not read line.");

        let lowercase_input = fahr_to_cel.trim().to_lowercase();
        let input_condition = lowercase_input.eq("y") || lowercase_input.eq("n");
        if input_condition {
            if lowercase_input == "y" {
                convert_to_cel = true;
                break;
            } else {
                convert_to_cel = false;
                break;
            }
        } else {
            continue;
        }
    }

    loop {
        let mut temp = String::new();
        if convert_to_cel {
            println!("Enter Fahrenheit:");
        } else {
            println!("Enter Celcius:")
        }
        io::stdin()
            .read_line(&mut temp)
            .expect("Could not read line");

        let temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if convert_to_cel {
            let result = (temp - 32) * 5 / 9;
            println!("---------- RESULT ----------");
            println!("{}°F = {}°C", temp, result);
            println!("---------- RESULT ----------");
        } else {
            let result = (temp * 9 / 5) + 32;
            println!("---------- RESULT ----------");
            println!("{}°C = {}°F", temp, result);
            println!("---------- RESULT ----------");
        }
        break;
    }
}

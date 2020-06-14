use std::io;

fn main() {
    println!("The computer will guess your number!");
    println!("Please choose a number between 1 and 100.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        if guess < 1 || guess > 100 {
            println!("The value must be between 1 and 100");
            continue;
        } else {
            break;
        }
    }
    println!("Please enter l for larger, s for smaller, r for right.");

    let mut upper_bound: u8 = 100;
    let mut lower_bound: u8 = 1;
    loop {
        let trial: u8 = (upper_bound - lower_bound) / 2 + lower_bound;

        println!("I guess {}", trial);
        loop {
            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Unable to read answer.");

            let answer: char = match answer.trim().to_lowercase().parse() {
                Ok(chr) => chr,
                Err(_) => {
                    println!("Must be a single character");
                    continue;
                }
            };

            if answer == 's' {
                upper_bound = trial;
                break;
            }

            if answer == 'l' {
                lower_bound = trial;
                break;
            }

            if answer == 'r' {
                println!("Right, the number is {}", trial);
                std::process::exit(0);
            }
            println!("{} is not a vaild input.", answer);
        }
    }
}

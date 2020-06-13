use std::io;

fn main() {
    let mut guess = String::new();

    println!("Please enter a guess for a number between 1 and 100:");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

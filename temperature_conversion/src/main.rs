use std::io;
use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    let temperature = input::<String>("Please enter a temperature followed by either \
    F for Fahrenheit or C for Celsius.");
    println!("{}", temperature);
}

pub fn input<T>(prompt: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let buffer : T = buffer.trim().parse().expect("Failed to parse.");
    buffer
}

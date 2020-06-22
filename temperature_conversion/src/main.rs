use std::any::type_name;
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let temperature = input_parse_retry::<f32>("Please enter a temperature: ", "Please try again.");
    println!("The temperature you entered was {}", temperature);
}

pub fn input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn input_parse<T>(prompt: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + 'static,
    <T as FromStr>::Err: Error + 'static,
{
    let buffer = input(prompt)?;
    Ok(buffer.trim().parse::<T>()?)
}

pub fn input_retry(prompt: &str, retry: &str) -> String {
    let string: String;
    loop {
        string = match input(prompt) {
            Ok(string) => string,
            Err(_) => {
                println!("Failed to parse into {}.", type_name::<String>());
                println!("{}", retry);
                continue;
            }
        };
        break;
    }
    string
}

pub fn input_parse_retry<T>(prompt: &str, retry: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug + 'static,
    <T as FromStr>::Err: Error + 'static,
{
    let number: T;
    loop {
        number = match input_parse::<T>(prompt) {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse into {}.", type_name::<T>());
                println!("{}", retry);
                continue;
            }
        };
        break;
    }
    number
}

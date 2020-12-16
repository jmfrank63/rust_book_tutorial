use std::collections::HashMap;
use std::io::{stdin, Result};

fn main() {
    let mut hmap: HashMap<&str, &str> = HashMap::new();
    {
        let buffer = loop {
            let mut buffer = String::new();
            match user_input("Please add a person to a department", &mut buffer) {
                Ok(_) => break buffer,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
        };

        let buffer = buffer.to_owned();
        let command = parse_input(&buffer);
        match &command[0].to_lowercase()[..] {
            "add" => add_to_department(&mut hmap, command),
            "remove" => remove_from_department(&mut hmap, command),
            _ => println!("Unknown command {}", command[0]),
        }

        println!("{:?}", hmap);
    }
}

fn user_input<'a>(prompt: &str, buffer: &'a mut String) -> Result<&'a str> {
    println!("{} :", prompt);
    stdin().read_line(buffer)?;
    if buffer.chars().any(char::is_numeric) {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No numbers are allowed!",
        ))
    } else {
        Ok(buffer)
    }
}

fn parse_input(buffer: &str) -> [&str; 3] {
    let command: Vec<&str> = buffer.split(' ').collect();
    // ignore "to" or "from"
    [command[0], command[1], command[3].trim()]
}

fn add_to_department<'a>(hmap: &mut HashMap<&'a str, &'a str>, command: [&'a str; 3]) {
    hmap.insert(command[2], command[1]);
    println!("Added {:?}", command);
}

fn remove_from_department<'a>(hmap: &mut HashMap<&'a str, &'a str>, command: [&'a str; 3]) {
    hmap.remove(command[2]);
    println!("Removed {:?}", command);
}

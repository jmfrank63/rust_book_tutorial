use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{stdin, Result, Error, ErrorKind};



fn main() {
    let mut hmap: HashMap<&str, &str> = HashMap::new();
    let mut input: String;

    loop {
        input = String::new();
        stdin().read_line(&mut input).expect("Unable to get input!");
        input = input.trim().to_owned();
        if input.is_empty() {
            break;
        };
        let command = match parse_input(&input[..]) {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {} Please try again.", e);
                continue;
            }
        };
        println!("{:?}", command);
    }
}

fn parse_input(input: &str) -> Result<[&str; 4]> {
    let words: Vec<&str> = input.split(' ').collect();
    match words.try_into(){
        Ok(mut arr) => {
            for (count, element) in arr.iter().enumerate() {
                println!("{}{}", count, element);
            }
            Ok(arr)
        },
        Err(v) => {
            let error = format!("Expected 4 but found {} arguments!", v.len());
            Err(Error::new(ErrorKind::Other, error))
        },
    }
}

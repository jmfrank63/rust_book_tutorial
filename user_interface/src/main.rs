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
        hmap.insert(command[3],command[1] );
    }
    println!("{:?}", hmap);
}

fn parse_input(input: &str) -> Result<[&str; 4]> {
    let words: Vec<&str> = input.split(' ').collect();
    match words.try_into(){
        Ok(arr) => {
            let a: [&str; 4] = arr;
            for (c, e) in a.iter().enumerate() {
                println!("{}: {}",c, e);
            }
            Ok(a)
        },
        Err(v) => {
            let error = format!("Expected 4 but found {} arguments!", v.len());
            Err(Error::new(ErrorKind::Other, error))
        },
    }
}

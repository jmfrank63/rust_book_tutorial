use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{stdin, Error, ErrorKind, Result};

fn main() {
    let mut hmap: HashMap<&str, &str> = HashMap::new();

    let command = loop {
        let mut input = String::new();
        let input = loop {
            stdin().read_line(&mut input).expect("Unable to get input!");
            input = input.trim().to_string();
            if input.is_empty() {
                break input;
            }
        };

        match parse_input(input) {
            Ok(c) => break c,
            Err(e) => {
                println!("Error: {} Please try again.", e);
                continue;
            }
        };
    };
    hmap.insert(&command[3][..], &command[1][..]);
    println!("{:?}", hmap);
}

fn parse_input(input: String) -> Result<[String; 4]> {
    let words: Vec<&str> = input.split(' ').collect();
    // match words.try_into() {
    //     Ok(arr) => {
    //         let a: [String; 4] = arr;
    //         for (c, e) in a.iter().enumerate() {
    //             println!("{}: {}", c, e);
    //         }
    //         Ok(a)
    //     }
    //     Err(v) => {
    //         let error = format!("Expected 4 but found {} arguments!", v.len());
    //         Err(Error::new(ErrorKind::Other, error))
    //     }
    // }
    Ok([String::from("Add"), String::from("Linda"), String::from("to"), String::from("Engineering")])
}

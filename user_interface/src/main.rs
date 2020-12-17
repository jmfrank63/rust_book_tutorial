use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{stdin, Result};

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
        let command = parse_input(&input[..]);
        println!("{:?}", command);
    }
}

fn parse_input(input: &str) -> Result<[&str; 3]> {
    let mut v: Vec<&str> = input.split(' ').collect();
    v.remove(2);
    let action :[&str;3] = v.try_into()
    .unwrap_or_else(|v: Vec<&str>|{println!("Not good!");["","",""]});
    Ok(action)
}

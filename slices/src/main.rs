fn main() {
    let words = String::from("Hello, world! ");
    let word = first_word(&words);
    println!("{}", word);
    let sword = second_word(&words);
    println!("{}", sword);
    println!("{}", words);
    let s = String::new();
    println!("{}", s);
    let q = String::new();
    println!("{}", q);
    let p = String::new();
    println!("{}", p);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut positions = Vec::new();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            positions.push(i);
        }
    }
    match positions.len() {
        2 => return &s[positions[0] + 1..positions[1]],
        _ => return &s[..],
    }
}

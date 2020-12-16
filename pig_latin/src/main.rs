use std::io;
fn main() {
    println!("Please type a word:");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => println!("I read {} bytes", n),
        Err(err) => println!("Could not read the line with error {}", err),
    }

    let mut word: Vec<char> = buffer.trim().chars().collect();

    match word[0] {
        'a'|'e'|'i'|'o'|'u' => {
            word.push('-');
            word.push('h');
            word.push('a');
            word.push('y');
        },
        _ => {
            let first  = word.remove(0);
            word.push('-');
            word.push(first);
            word.push('a');
            word.push('y');
        }
    }
    let pig_word: String = word.iter().collect();

    println!("The pig-latin word is {}", pig_word);
}

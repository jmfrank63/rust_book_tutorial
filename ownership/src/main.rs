fn main() {
    mutable_string();
    move_variables();
    move_string();
    copy_string();
}

fn mutable_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn move_variables() {
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
}

fn move_string() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}

fn copy_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
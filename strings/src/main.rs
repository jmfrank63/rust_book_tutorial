use strings;
fn main() {
    strings::strings();

    let hello = "Здравствуйте";

    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();
    
    let s3 = strings::s_from() + " " + &strings::s_to();

    println!("{}", s3);
}
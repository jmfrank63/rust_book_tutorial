use std::io::{Result, stdin};

fn main()-> Result<()>{
    let mut buffer = String::new();
    user_input("Please add a person to a department", &mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn user_input<'a>(prompt: &str, buffer: &'a mut String) -> Result<&'a str> {
    println!("{} :", prompt);
    stdin().read_line(buffer)?;
    Ok(buffer)
}

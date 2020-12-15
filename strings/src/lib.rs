pub fn strings() {
    s_to();
    s_from();
}

pub fn s_to() -> String {
    "Hello, world!".to_string()
}

pub fn s_from() -> String {
    String::from("Hello, world!")
}

fn main() {
    let mut v: Vec<isize> = Vec::new();
    
    for i in (1..10).rev() {
        v.push(i);
    }

    let third: &isize = &v[2];
    println!("{}", third);

    match v.get(11) {
        Some(n) => println!("{}", n),
        None => println!("No element at this index"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }


    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

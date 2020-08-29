fn main() {
    let v1: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let first = &v[0];
    println!("The first element is: {}", first);
    v.push(6);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(5.555555),
        SpreadsheetCell::Text(String::from("yeeeeehaw")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
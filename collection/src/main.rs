use std::collections::HashMap;

fn main() {
    //Vectors

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector is {:?}", v);

    let mut v1 = vec![1, 2, 3];
    println!("vector is {:?}", v1);

    let third = &v[2];

    //Error
    // let does_not_exist = &v[100];

    println!("The third element is {}", third);

    match v1.get(100) {
        Some(i) => println!("We got this {:?}", i),
        None => println!("Index does not exist"),
    }

    match v1.get(2) {
        Some(i) => println!("We got this {:?}", i),
        None => println!("Index does not exist"),
    }

    println!("\n Immutable reference\n");

    for i in &v1 {
        println!("Vector is {:?}", i);
    }

    println!("\n Mutable reference\n");

    for i in &mut v1 {
        *i += 20;
        println!("Vector is {:?}", i);
    }

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

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Here is {:?}", i),
        _ => println!("Not a integer"),
    }

    //String

    let mut str = String::new();
    str.push('h');
    println!("String is {:?}", str);

    let mut str1 = String::from("Hello");
    str1.push_str(" World");
    str1.push('!');
    println!("String is {str1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //

    println!("String is {s3}");

    //Hash Map

    println!("\n");

    let mut hash = HashMap::new();
    hash.insert(1, 20);
    println!("Hash map is {:?}", hash);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(&String::from("Yellow")).copied().unwrap_or(0);

    println!("Score is {:?}\n", score);

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for char in text.split_whitespace() {
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

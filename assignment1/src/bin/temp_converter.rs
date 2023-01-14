use std::io;

fn main() {
    println!("Temperature Converter");

    println!("Please input a temperature in Fahrenheit: ");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f32=match fahrenheit.trim().parse(){
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{} degrees Fahrenheit is {} degrees Celsius", fahrenheit, celsius);
}

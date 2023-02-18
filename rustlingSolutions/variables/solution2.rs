// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

// I AM DONE
use std::io;

fn main() {
    /**
     * Can also be solved by this
     * let x=10;
     */
    println!("Enter a number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error in input");

    let x = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Please enter a valid value");
            0
        }
    };

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

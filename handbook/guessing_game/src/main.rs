use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
   println!("Guess the number!");


   let  random_number=rand::thread_rng().gen_range(1..101);

   println!("The secret number is: {}", random_number);

   loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32=match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}","Too small!".red().bold()),
            Ordering::Greater => println!("{}","Too big!".red().bold()),
            Ordering::Equal =>{ 
                println!("{}","You win!".green().bold());
                break;
            }
        }
   }
  
}

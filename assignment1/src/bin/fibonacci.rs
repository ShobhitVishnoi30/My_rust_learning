use std::io;
use colored::*;

fn main(){
   println!("Fibonacci Sequence");


   println!("Enter fibonacci series length");

    let mut length = String::new();

    io::stdin().read_line(&mut length).expect("Error in input");

    let length: u32=match length.trim().parse(){
        Ok(num) => num,
        Err(_) => {
             println!("{}","Please enter a valid length".red());
             0
        },
    };

    const FIRST_NUMBER:i32=0;
    const SECOND_NUMBER:i32=1;
    println!("{}","Here is fibonacci series".green());
    if length==1{
        println!("{}",FIRST_NUMBER);
    }else if length==2{
        println!("{} \n{}",FIRST_NUMBER ,SECOND_NUMBER);
    }else if length>2{
        let mut first_number=FIRST_NUMBER;
        let mut second_number=SECOND_NUMBER;
        let mut current_number;
        let mut i=0;
       
        println!("{} \n{}",FIRST_NUMBER ,SECOND_NUMBER);

        while i<length {
           current_number=first_number+second_number;
           first_number=second_number;
           second_number=current_number;
           println!("{}",current_number);
           i =i+1;
        }
    }
}
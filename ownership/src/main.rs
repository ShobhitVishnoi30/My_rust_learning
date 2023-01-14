fn main() {

   
   let s1=String::from("Hello");
   let s2=s1; //Move or shallow copy

   println!("{}",s2);



   let a1=String::from("hello");
   let a2=a1.clone();

   println!("a1 = {}, a2 = {}", a1, a2);

    let s = String::from("hello");  // s comes into scope

   // takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

   //println!("{}",s);  // throws error

    takes_ownership(s.clone());

    println!("{}",s);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    println!("{}",x);



    let s1 = String::from("hello world");

    let len = calculate_length(&s1);  // passing reference

    //calculate_length(s1) then s1 will be dropped

    println!("The length of '{}' is {}.", s1, len);


} 

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} 

//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main(){
   twelve_days_of_christmas();
}

fn twelve_days_of_christmas(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    for i in 0..12{
        println!("On the {} day of Christmas my true love sent to me", days[i]);
        for j in (0..i+1).rev(){
           
            if i == 0{
                println!("{}", gifts[j]);
            }
            else if j == 0{
                println!("and {}", gifts[j]);
            }
            else{
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}
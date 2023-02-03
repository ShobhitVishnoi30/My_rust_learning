struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn getString(&self, s: &'a str) -> &'a str {
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let a = ImportantExcerpt { part: &string1 };

    let string2 = String::from("xyz");
    println!("The longest string is {}", a.getString(&string2));

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);

    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }

    //The following code will not compile because the result variable is no longer valid

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
}

//'a is a lifetime parameter
//The lifetime of the reference returned by the longest function is the same
//as the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

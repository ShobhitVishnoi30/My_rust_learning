#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Guess {
    a: u32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { a: value as u32 }
    }
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: u32, b: u32) -> u32 {
    return a + b;
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2, 2));
    }

    #[test]
    fn it_adds_two_failed() {
        assert_ne!(5, add_two(2, 2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", //Custom error message
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn test_guess() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn test_guess_100() {
        Guess::new(101);
    }

    #[test]
    fn test_guess_50() {
        Guess::new(50);
    }

    #[test]
    fn test_result_type() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

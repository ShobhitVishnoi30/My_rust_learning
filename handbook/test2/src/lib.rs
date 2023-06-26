struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greetings(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    // #[test]
    // fn it_fails() {
    //     panic!("made to fail")
    // }

    #[test]
    fn can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };

        let smaller = Rectangle {
            width: 2,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };

        let smaller = Rectangle {
            width: 2,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greet() {
        let result = greetings("Shobhit");
        assert!(result.contains("Shobhit"));
    }

    #[test]
    fn greet_should_fail() {
        let result = greetings("aa");
        assert!(
            result.contains("Shobhit"),
            "do not contains shobhit => {}",
            result
        );
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 25,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 60,
    };

    let square = Rectangle::square(20);

    println!("Sqaure is {:#?}", square);

    println!("Area of the rectangle is {}", rect_area(&rect));

    println!("Area of the rectangle is {}", rect.area());

    println!("Rectangle 1 can hold Rectangle 2 {}", rect.can_hold(rect2));

    println!("Rectangle 1 can hold Rectangle 3 {}", rect.can_hold(rect3));
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

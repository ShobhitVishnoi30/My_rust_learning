pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Default implementation")
    }
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("Point is x: {}, y: {}", self.x, self.y)
    }
}

impl Summary for Circle {}

pub fn area(shape: &impl Summary) -> String {
    shape.summarize()
}

pub fn area_with_generic<T: Summary>(shape: &T) -> String {
    shape.summarize()
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    let c = Circle { radius: 3.0 };
    let d = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    println!("Point is {:?}", p.summarize());

    println!("Point is {:?}", c.summarize());

    println!("Summart of area p {:?}", area(&p));

    println!("Summart of area p {:?}", area_with_generic(&p));
}

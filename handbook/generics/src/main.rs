//support only one data type same for both
#[derive(Debug)]
struct Gen<T> {
    a: T,
    b: T,
}

//Support different data type for both fields
#[derive(Debug)]
struct GenD<T, U> {
    a: T,
    b: U,
}

impl<T: Copy, U: Copy> GenD<T, U> {
    fn a(&self) -> T {
        self.a
    }

    fn b(&self) -> U {
        self.b
    }
}

impl<T, U> GenD<T, U> {
    fn mixup<V, W>(self, other1: GenD<V, W>) -> W {
        // GenD {
        //     a: self.a,
        //     b: other1.b,
        // }

        other1.b
    }
}

fn main() {
    let int_array = vec![1, 4, 5, 10, 5];

    let largest = get_largest(int_array);

    println!("Largest element is {:?}", largest);

    let char_array = vec!['a', 'b', 'c', 'a', 'e'];

    let largest = get_largest(char_array);

    println!("Largest element is {:?}", largest);

    let g = Gen { a: 2, b: 5 };

    println!("Struct is {:?}", g);

    let g = GenD { a: 2, b: 5.0 };

    println!("Struct is {:?}", g);

    println!("Calling implementation {:?}", g.a());

    let g = GenD { a: 2, b: 5.0 };

    let g1 = GenD { a: "Hello", b: 'c' };

    println!("Calling implementation {:?}", g.mixup(g1));
}

fn get_largest<T: PartialOrd + Copy>(ar: Vec<T>) -> T {
    let mut largest: T = ar[0];

    for current in ar {
        if current > largest {
            largest = current;
        }
    }
    largest
}

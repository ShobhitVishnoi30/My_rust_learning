fn main() {
    let int_array = vec![1, 4, 5, 10, 5];

    let largest = get_largest(int_array);

    println!("Largest element is {:?}", largest);

    let char_array = vec!['a', 'b', 'c', 'a', 'e'];

    let largest = get_largest(char_array);

    println!("Largest element is {:?}", largest);
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

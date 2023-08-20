pub fn check_palindrome(a: u32) -> bool {
    let mut reversed_number = 0;
    let mut original_number = a;
    while original_number > 0 {
        let mut b = original_number % 10;
        reversed_number = reversed_number * 10 + b;
        original_number = original_number / 10;
    }

    if a == reversed_number {
        return true;
    } else {
        return false;
    }
}

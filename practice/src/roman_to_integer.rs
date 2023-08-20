
pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut previous: char = ' ';
    for c in s.chars() {
        if (previous != ' ' && calculate_number(previous) < calculate_number(c)) {
            sum = sum + calculate_number(c) - (2 * calculate_number(previous));
        } else {
            sum = sum + calculate_number(c);
        }
        previous = c;
    }
    return sum;
}

pub fn calculate_number(c: char) -> i32 {
    if c == 'I' {
        return 1;
    } else if c == 'V' {
        return 5;
    } else if c == 'X' {
        return 10;
    } else if c == 'L' {
        return 50;
    } else if c == 'C' {
        return 100;
    } else if c == 'D' {
        return 500;
    } else if c == 'M' {
        return 1000;
    } else {
        return 0;
    }
}

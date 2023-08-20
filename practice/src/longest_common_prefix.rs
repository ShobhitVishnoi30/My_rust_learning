pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut longest = strs[0].clone();

    for next_string in &strs[1..] {
        longest = find_common_prefix(&longest, next_string);
    }

    return longest;
}

fn find_common_prefix(s1: &str, s2: &str) -> String {
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    let mut common_end = 0;
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) if c1 == c2 => {
                common_end += 1;
            }
            _ => break,
        }
    }

    s1[0..common_end].to_string()
}

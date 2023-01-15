use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique = HashSet::new();
    for i in 0..nums.len() {
        if (unique.contains(&nums[i])) {
            return true;
        }
        unique.insert(nums[i]);
    }
    false
}

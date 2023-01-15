use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    let mut result = Vec::new();
    for i in 0..nums.len() {
        if hash_map.contains_key(&(target - nums[i])) {
            let b = match hash_map.get(&(target - nums[i])) {
                Some(a) => *a,
                None => 0,
            };
            result.push(b as i32);
            result.push(i as i32);
            break;
        }
        hash_map.insert(nums[i], i);
    }
    result
}

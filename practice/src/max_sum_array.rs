
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum: i32 = 0;
    let mut current_sum: i32 = 0;

    for i in 0..nums.len() {
        current_sum = current_sum + nums[i];

        if (current_sum > max_sum || i == 0) {
            max_sum = current_sum;
        }

        if current_sum < 0 {
            current_sum = 0
        }
    }
    max_sum
}

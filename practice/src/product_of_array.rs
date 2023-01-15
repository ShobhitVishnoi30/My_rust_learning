
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix: i32 = 1;
    let mut postfix: i32 = 1;
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        result.push(prefix);
        prefix = prefix * nums[i];
    }

    for i in (0..nums.len()).rev() {
        result[i] = result[i] * postfix;
        postfix = postfix * nums[i];
    }
    result
}

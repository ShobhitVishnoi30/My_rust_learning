pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        if (nums[left] < nums[right]) {
            if (min > nums[left]) {
                min = nums[left]
            }
            break;
        }
        let mid = (left + right) / 2;
        if (nums[mid] < min) {
            min = nums[mid];
        }

        if (nums[left] <= nums[mid]) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return min;
}

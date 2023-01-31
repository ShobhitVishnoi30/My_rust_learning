pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut min: i32 = nums[0];
    let mut right: usize = nums.len() - 1;
    let mut left = 0;

    if (right == 0) {
        return nums[0];
    }

    while left <= right {
        if (nums[left] < nums[right]) {
            if (min > nums[left]) {
                min = nums[left];
            }

            break;
        }
        let mid = (left + right) / 2;
        if (min > nums[mid]) {
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

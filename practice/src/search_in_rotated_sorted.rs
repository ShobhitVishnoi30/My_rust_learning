use core::num;

pub fn search_sorted(nums: Vec<i32>, target: i32) -> i32 {
    let mut minIndex = find_min_index(nums.clone());

    let mut left = 0;
    let mut right = nums.len() - 1;
    if (nums.len() == 1) {
        if (nums[0] != target) {
            return -1;
        }
    }

    if (nums[minIndex] == target) {
        return minIndex as i32;
    } else {
        if (nums[right] == target) {
            return right as i32;
        }
        if (nums[left] == target) {
            return left as i32;
        }
        println!("23");
        if (nums[right] > target) {
            left = minIndex + 1;
        } else {
            if (minIndex == 0) {
                return -1;
            }
            right = minIndex - 1;
        }
    }
    println!("30");
    while left <= right {
        let mut mid = (left + right) / 2;

        if (nums[mid] == target) {
            return mid as i32;
        }
        if (mid == 0 || mid == nums.len() - 1) {
            return -1;
        }
        if (nums[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return -1;
}

pub fn find_min_index(nums: Vec<i32>) -> usize {
    let mut min = nums[0];
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut index = 0;

    while left <= right {
        if (nums[left] < nums[right]) {
            if (min > nums[left]) {
                min = nums[left];
                index = left;
            }
            break;
        }
        let mid = (left + right) / 2;
        if (nums[mid] < min) {
            min = nums[mid];
            index = mid;
        }

        if (nums[left] <= nums[mid]) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return index;
}

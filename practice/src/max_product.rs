pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_prod: i32 = max_element(&nums);
    let mut current_min: i32 = 1;
    let mut current_max: i32 = 1;
    let mut flag = false;

    for i in 0..nums.len() {
        if (nums[i] == 0) {
            flag = true;
            current_min = nums[i];
            current_max = nums[i];
        } else {
            let temp_min = current_min * nums[i];
            let temp_max = current_max * nums[i];

            let vc = vec![temp_min, temp_max, nums[i]];
            current_max = max_element(&vc);
            current_min = min_element(&vc);

            if current_max > max_prod {
                max_prod = current_max;
            }
        }
    }
    if flag {
        if max_prod > 0 {
            return max_prod;
        } else {
            return 0;
        }
    }

    return max_prod;
}

pub fn max_element(vec: &Vec<i32>) -> i32 {
    let mut max = vec[0];

    for &elem in vec.iter().skip(1) {
        if elem > max {
            max = elem;
        }
    }

    max
}

pub fn min_element(vec: &Vec<i32>) -> i32 {
    let mut min = vec[0];

    for &elem in vec.iter().skip(1) {
        if elem < min {
            min = elem;
        }
    }

    min
}

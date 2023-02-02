mod buy_sell_stock;
mod contains_duplicate;
mod max_product;
mod max_sum_array;
mod minimum_sorted_rotated;
mod product_of_array;
mod two_sum;

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 1];
    let result = contains_duplicate::contains_duplicate(nums);

    print!("Array contains duplicate {result}\n");

    let nums = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result = two_sum::two_sum(nums, target);

    print!("Target found {:?}\n", result);

    let nums = vec![7, 1, 5, 3, 6, 4];

    let profit = buy_sell_stock::max_profit(nums);

    print!("Max profit is {:?}\n", profit);

    let nums = vec![-1, 1, 0, -3, 3];

    let result_array = product_of_array::product_except_self(nums);
    print!("Product array is {:?}\n", result_array);

    let nums = vec![5, 4, -1, 7, 8];
    let result = max_sum_array::max_sub_array(nums);

    println!("Maximum subarray sum is {result}\n");

    let nums = vec![-1, -1, -2, -2];
    let result = max_product::max_product(nums);
    println!("Maximum product is {result}\n");

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let result = minimum_sorted_rotated::find_min(nums);
    println!("Minimum in Rotated Sorted Array is {result}\n");
}

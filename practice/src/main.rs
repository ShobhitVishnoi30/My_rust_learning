mod buy_sell_stock;
mod contains_duplicate;
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
}

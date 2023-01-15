pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut l: usize = 0;
    let mut r: usize = 0;
    while r < prices.len() {
        if (prices[l] < prices[r]) {
            let x = prices[r] - prices[l];
            if max < x {
                max = x;
            }
        } else {
            l = r;
        }
        r = r + 1;
    }
    max
}

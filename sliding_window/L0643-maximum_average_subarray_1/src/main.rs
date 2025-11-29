//! Leetcode: 0643-Maximum Average Subarray I
//! Link: https://leetcode.com/problems/maximum-average-subarray-i/
//! Category: sliding window
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%

fn main() {
    // println!("{}", find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
    println!("{}", find_max_average(vec![0, 1, 1, 3, 3], 4));
    println!("{}", find_max_average(vec![4, 0, 4, 3, 3], 5));
}

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut sum = nums[..k].iter().sum::<i32>();
    let mut max = sum;

    for i in k..nums.len() {
        sum += nums[i] - nums[i-k];
        max = max.max(sum);
    }

    max as f64 / k as f64
}

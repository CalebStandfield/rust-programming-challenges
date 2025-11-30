//! Leetcode: 0704-Binary Search
//! Link: https://leetcode.com/problems/binary-search
//!
//! Category: binary search
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Binary search is binary search.
//! No, but seriously it involves taking the problem, asking which half is the value on.
//! Move the high or low to the mid + or - 1 respectively.
//! Then split in half again.
//! Note: For Rust this is a ***** it's really annoying since your numbers can underfill if they are usize.
//! So you have to cast them all as `i32`, which is lame but makes sense.
//! Remember that it is while low <= high
//! Then calculate mid
//! Make an easy val variable.
//! Check which half it is on.
//! Less than -> move low to mid + 1
//! Greater than -> move high to mid - 1

fn main() {
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 12));
    println!("{}", search(vec![-1], -1));
    println!("{}", search(vec![5], -1));
    println!("{}", search(vec![], 2));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low <= high {
        let mid = (high + low) / 2;
        let val = nums[mid as usize];
        if val == target {
            return mid;
        }
        if val < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    -1
}

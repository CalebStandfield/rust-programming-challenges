//! Leetcode: 0001-Two Sum
//! Link: https://leetcode.com/problems/two-sum/
//! 
//! Category: Hash Table/Two Pointer
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//! 
//! Method of Solving: Use a HashMap since a two-pointer approach is O(n^2).
//! For i and num in nums calculate the difference between target and num.
//! If 'Some' value exists in map.get(num) do a check.
//! If i and j are equal, then continue since we can't return the same number.
//! Otherwise, return a new vec with i and j. 
//! If 'Some' was 'None' then insert into the map the difference and i.
//! At the end return the empty vector if no solution is found.

use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ans = two_sum(nums, target);
    println!("Answer: {:?}", ans);

    let nums = vec![3, 2, 4];
    let target = 6;
    let ans = two_sum(nums, target);
    println!("Answer: {:?}", ans);

    let nums = vec![3, 3];
    let target = 6;
    let ans = two_sum(nums, target);
    println!("Answer: {:?}", ans);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = map.get(num) {
            if i == j { continue; }
            return vec![j as i32, i as i32];
        }
        map.insert(diff, i);
    }
    vec![]
}

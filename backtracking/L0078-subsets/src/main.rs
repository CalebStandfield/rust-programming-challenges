//! Leetcode: 0078
//! Link: https://leetcode.com/problems/subsets
//!
//! Category: backtracking
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Backtracking.
//! Our base case this time around needs to pass in a recursive_count variable, if equal to length push and return.
//! Then remember to not overcomplicate it.
//! Modify -> Recurse -> Undo -> Recurse.
//! This question is much simpler than permutations, so again DO NOT overcomplicate it.

fn main() {
    println!("Hello, world!");
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // What we are returning
    let mut result: Vec<Vec<i32>> = Vec::new();
    // We need a current subset of nums
    let mut curr: Vec<i32> = Vec::new();

    fn recurse(
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
        recursive_count: usize,
    ) {
        // Base case
        // If the recursive call is number 3, we know that we should push the vector to result and return.
        if recursive_count == nums.len() {
            result.push(curr.clone());
            return;
        }

        curr.push(nums[recursive_count]);
        recurse(&nums, result, curr, recursive_count + 1);
        curr.pop();
        recurse(&nums, result, curr, recursive_count + 1);
    }

    recurse(&nums, &mut result, &mut curr, 0);

    result
}

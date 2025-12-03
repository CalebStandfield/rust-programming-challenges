//! Leetcode: 0046
//! Link: https://leetcode.com/problems/permutations
//!
//! Category: backtracking
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Backtracking.
//! Use a &mut result array to pass in.
//! Use a &mut used bool area for the indices used.
//! Use a &mut curr vec for the permutation state.
//! Make a recursive method that takes in all of those params.
//! Base case: if the length of curr is == nums.length.
//! Do a for loop for all numbers in nums, check if that number has already been used though.
//! Push the number at index to curr and set the bool array to be true.
//! Recursive call.
//! Then undo, very important. This goes back a branch and allows to backtrack.
//! Then return the result.

fn main() {
    println!("{:?}", permute(vec![1, 2, 3]));
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut used = vec![false; nums.len()];
    let mut curr: Vec<i32> = Vec::with_capacity(nums.len());

    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        // Base case
        if curr.len() == nums.len() {
            result.push(curr.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            curr.push(nums[i]);
            backtrack(result, curr, nums, used);
            used[i] = false;
            curr.pop();
        }
    }

    backtrack(&mut result, &mut curr, &nums, &mut used);

    result
}

//! Leetcode: 0503-Next Greater Element II
//! Link: https://leetcode.com/problems/next-greater-element-ii
//!
//! Category: stack, hashmap
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Monotonic stack. Loop twice
//! While there are items in the stack, Check if the number at nums[idx] is greater than nums[top].
//! If so, set the return array at the top to be equal to nums[idx].
//! Then pop the stack. Otherwise, break the loop and push to the stack.

fn main() {
    println!("{:?}", next_greater_elements(vec![1, 2, 1]));
    println!("{:?}", next_greater_elements(vec![1, 2, 3, 4, 3]));
}

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..2 * n {
        let idx = i % n;

        while let Some(&top) = stack.last() {
            if nums[idx] > nums[top] {
                res[top] = nums[idx];
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(idx);
    }

    res
}

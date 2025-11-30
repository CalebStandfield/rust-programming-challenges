//! Leetcode: 0033-Search in Rotated Sorted Array
//! Link: https://leetcode.com/problems/search-in-rotated-sorted-array
//!
//! Category: binary search
//! Level: Medium
//! Runtime: 0 ms | Beats 100%
//!
//! Method of Solving: Find out which side is sorted.
//! Example [4, 5, 6, 7, 0, 1, 2]. Here we see that [4, 5, 6, 7] is sorted.
//! Check the lower value to the middle value and determine which side is sorted.
//! Thus, we go down our first branch. Then we need to check if our target is inside this branch.
//! If it is, then we set high to mid - 1, if not, then we set low to mid + 1.
//! Exactly the same logic but reversed for if the right side was sorted and the left wasn't.
//! Remember that half the array is sorted, figure out which half and check if the value exists in that half.

fn main() {
    println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() as i32 - 1;

    while low <= high {
        let mid = (low + high) / 2;

        let low_val = nums[low as usize];
        let mid_val = nums[mid as usize];
        let high_val = nums[high as usize];

        if mid_val == target {
            return mid;
        }

        if low_val <= mid_val {
            if low_val <= target && target < mid_val {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if mid_val < target && target <= high_val {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    -1
}
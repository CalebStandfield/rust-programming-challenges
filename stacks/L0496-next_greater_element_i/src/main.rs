//! Leetcode: 0496-Next Greater Element I
//! Link: https://leetcode.com/problems/next-greater-element-i
//!
//! Category: stack, hashmap
//! Level: easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Monotonic stack.
//! While there are items in the stack, check if the item in the stack is less than the new Item.
//! If it is, then we insert it and pop the item out and then push that new number to the stack.
//! We go through all of nums2, and then any numbers left won't get mapped which we can account for.
//! When making the return list, we get the mapping for n or default to -1.

fn main() {
    println!("{:?}", next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]));
    println!("{:?}", next_greater_element(vec![2, 4], vec![1, 2, 3, 4]));
    println!("{:?}", next_greater_element(vec![2, 1, 3], vec![2, 3, 1]));
}

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::with_capacity(nums1.len());
    let mut stack = Vec::with_capacity(nums2.len());
    let mut map = std::collections::HashMap::new();

    for num in nums2 {
        while let Some(&top) = stack.last() {
            if top < num {
                map.insert(top, num);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(num);
    }

    for n in nums1 {
        let value = map.get(&n).copied().unwrap_or(-1);
        ret.push(value);
    }
    ret
}

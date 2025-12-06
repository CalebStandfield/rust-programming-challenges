//! Leetcode: 0112 Path Sum
//! Link: https://leetcode.com/problems/path-sum
//!
//! Category: graph bfs
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00
//!
//! Method of Solving: Recursive search on each node.
//! Take the current sum, see if this node + curr_sum is equal to the target.
//! If not, try children.

fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, curr_sum: i32) -> bool {
        if let Some(rc) = node {
            let node_ref = rc.borrow();
            let curr_sum = curr_sum + node_ref.val;

            if node_ref.left.is_none() && node_ref.right.is_none() {
                return curr_sum == target_sum;
            }

            let left = node_ref.left.clone();
            let right = node_ref.right.clone();

            helper(left, target_sum, curr_sum) || helper(right, target_sum, curr_sum)
        } else {
            false
        }
    }
    if root.is_none() {
        return false;
    }
    helper(root, target_sum, 0)
}

//! Leetcode: 0256 Binary Tree Paths
//! Link: https://leetcode.com/problems/binary-tree-paths
//!
//! Category: backtracking trees
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Use backtracking.
//! As a safety measure, ensure node is Some.
//! If the node.left and node.right are both null, then take the current path append "->" and push to the result.
//! Then push the value and the recursive method on left and right.

fn main() {}

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
      right: None
    }
  }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths = Vec::new();
    let mut curr = Vec::new();

    fn find_paths(node: &Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, curr: &mut Vec<String>) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                let mut string = node.val.to_string();
                string.push_str("->");
                return;
            }
            curr.push(node.val.to_string());
            find_paths(&node.left, paths, curr);
            find_paths(&node.right, paths, curr);
            curr.pop();
        }
    }

    find_paths(&root, &mut paths, &mut curr);
    paths
}

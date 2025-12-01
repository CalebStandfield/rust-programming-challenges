//! Leetcode: 0206-Reverse Linked List
//! Link: https://leetcode.com/problems/reverse-linked-list/
//!
//! Category: linked list
//! Level: easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Reverse the way the list points.
//! Take prev, curr, and node.next.
//! `curr` will be set to head.
//! While there are more nodes set, Some node = curr.
//! Let curr be equal to that node.next. (remember to use .take() to extract the value out of the option)
//! Let node.next = prev.
//! Prev = Some(node).

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let head = vec_to_list(nums);
    println!("{:?}", reverse_list(head));
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}


fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;

    for n in nums {
        curr.next = Some(Box::new(ListNode::new(n)));
        curr = curr.next.as_mut().unwrap();
    }

    dummy.next
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//! Leetcode: 0002-Add Two Numbers
//! Category: linked lists
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Notes: opt.and_then(|x| n.next) consumes 'opt', runs the closure only if Some and returns the Option from the closure.
//! Keep 2 pointers to the return list. One for moving and creating, one to return -> dummy.next.

fn main() {
    let l1 = vec_to_list(vec![2, 4, 3]);
    let l2 = vec_to_list(vec![5, 6, 4]);
    println!("{:?}", add_two_numbers(l1, l2));
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut curr_l1 = l1;
    let mut curr_l2 = l2;

    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;
    let mut carry = 0;
    while curr_l1.is_some() || curr_l2.is_some() || carry > 0 {
        let mut a = 0;
        if curr_l1.is_some() {
            a = curr_l1.as_ref().unwrap().val;
        }
        let mut b = 0;
        if curr_l2.is_some() {
            b = curr_l2.as_ref().unwrap().val;
        }
        let mut val = a + b + carry;
        if val > 9 {
            carry = 1;
            val %= 10;
        } else {
            carry = 0;
        }

        curr_l1 = curr_l1.and_then(|n| n.next);
        curr_l2 = curr_l2.and_then(|n| n.next);

        curr.next = Some(Box::new(ListNode::new(0)));
        curr = curr.next.as_mut().unwrap();
        curr.val = val;


    }
    dummy.next
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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

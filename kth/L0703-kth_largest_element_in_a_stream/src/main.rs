//! Leetcode: 0703 kth largest element in a stream
//! Link: https://leetcode.com/problems/kth-largest-element-in-a-stream
//!
//! Category: kth
//! Level: easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: If we only need the kth largest, we can use a min heap.
//! First, implement add() so you can call add() in new. 
//! In add() it's as simple as pushing to the heap and then checking if that exceeds k.
//! If so, pop the element. Call peak() at the end to return the i32.
//! Note: A min heap is a BinaryHeap<Reverse<i32>>.
//! When peaking, remember to call .unwrap().0 where .0 gets the field out of Reverse.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let k = 3;
    let nums = vec![4, 5, 8, 2];
    let mut obj = KthLargest::new(k, nums);
    let mut vec = Vec::new();
    vec.push(obj.add(3));
    vec.push(obj.add(5));
    vec.push(obj.add(10));
    vec.push(obj.add(9));
    vec.push(obj.add(4));
    println!("{:?}", vec);
}

struct KthLargest {
    k: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            k,
            heap: BinaryHeap::new(),
        };

        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

// /**
//  * Your KthLargest object will be instantiated and called as such:
//  * let obj = KthLargest::new(k, nums);
//  * let ret_1: i32 = obj.add(val);
//  */
//! Leetcode: 0215 Largest Element in an Array
//! Link: https://leetcode.com/problems/kth-largest-element-in-an_array/
//!
//! Category: kth
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Use a min heap.
//! Create a struct to make things easier, the struct contains a capacity and a BinaryHeap<Reverse<i32>>.
//! Then keep the capacity to k.
//! Add elements to the min heap and then peek() the last element.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let vec = vec![3, 2, 1, 5, 6, 4];
    println!("{}", find_kth_largest(vec, 2))
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {

    let heap = MinHeap::new(k as usize, nums);
    heap.find_kth_largest()
}

struct MinHeap {
    capacity: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl MinHeap {
    fn new(capacity: usize, nums: Vec<i32>) -> MinHeap {
        let mut minheap = MinHeap {
            capacity,
            heap: BinaryHeap::with_capacity(capacity),
        };
        for num in nums {
            minheap.add(num);
        }
        minheap
    }

    fn add(&mut self, num: i32) {
        if self.heap.len() < self.capacity {
            self.heap.push(Reverse(num));
        } else if self.heap.peek().unwrap().0 < num {
            self.heap.pop();
            self.heap.push(Reverse(num));
        }

    }

    fn find_kth_largest(&self) -> i32 {
        self.heap.peek().unwrap().0
    }
}

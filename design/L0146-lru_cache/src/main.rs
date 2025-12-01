//! Leetcode: 0146 LRU Cache
//! Link: https://leetcode.com/problems/lru-cache/
//!
//! Category: design
//! Level: Medium
//! Runtime:  ms | Beats %
//!
//! Method of Solving:

use std::collections::{HashMap, LinkedList};

fn main() {
    let mut obj = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    let ret_1: i32 = obj.get(1);
    println!("{:?}", ret_1);
    obj.put(3, 3);
    let ret_2: i32 = obj.get(2);
    println!("{:?}", ret_2);
    obj.put(4, 4);
    let ret_3: i32 = obj.get(1);
    println!("{:?}", ret_3);
    let ret_3: i32 = obj.get(3);
    let ret_4: i32 = obj.get(4);
    println!("{} {}", ret_3, ret_4);
}

struct LRUCache {
    capacity: i32,
    list: LinkedList<i32>,
    map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            list: LinkedList::new(),
            map: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            let mut removed_one = false;
            self.list
                .extract_if(|x| {
                    if !removed_one && *x == key {
                        removed_one = true;
                        true
                    } else {
                        false
                    }
                })
                .for_each(drop);

            self.list.push_front(key);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.map.insert(key, value);

        let mut removed_one = false;
        self.list
            .extract_if(|x| {
                if !removed_one && *x == key {
                    removed_one = true;
                    true
                } else {
                    false
                }
            })
            .for_each(drop);

        self.list.push_front(key);

        if self.list.len() > self.capacity as usize {
            if let Some(lru_key) = self.list.pop_back() {
                self.map.remove(&lru_key);
            }
        }
    }
}

// /**
//  * Your LRUCache object will be instantiated and called as such:
//  * let obj = LRUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */

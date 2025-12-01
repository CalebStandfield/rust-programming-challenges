//! Leetcode: 0146 LRU Cache
//! Link: https://leetcode.com/problems/lru-cache/
//!
//! Category: design
//! Level: Medium
//! Runtime: 29 ms | Beats 97.74%
//!
//! Method of Solving: Using a HashMap and a custom Doubly-LinkedList.
//! The Idea is using a HashMap for O(1) access to any element.
//! Then use Rc<RefCell<CustomNode>> as the value inside the HashMap.
//! This allows you to get any node from the List in O(1) instead of O(n) for list traversal.

// /**
//  * Your LRUCache object will be instantiated and called as such:
//  * let obj = LRUCache::new(capacity);
//  * let ret_1: i32 = obj.get(key);
//  * obj.put(key, value);
//  */
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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LRUCache {
    capacity: i32,
    list: DoubleLinked,
    map: HashMap<i32, Rc<RefCell<DoubleLinkedNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            list: DoubleLinked::new(None, None),
            map: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.map.contains_key(&key) {
            return -1;
        }
        let node = self.map.get(&key).unwrap();
        self.list.move_head(node.clone());
        self.list.get_head().unwrap().borrow().value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.list.move_head(node);
            return;
        }

        if self.map.len() == self.capacity as usize {
            if let Some(tail) = self.list.get_tail() {
                let k = tail.borrow().key;
                self.list.remove(tail);
                self.map.remove(&k);
            }
        }

        let node = Rc::new(RefCell::new(DoubleLinkedNode::new(key, value)));
        self.list.push_front(node.clone());
        self.map.insert(key, node);
    }
}

struct DoubleLinked {
    head: Option<Rc<RefCell<DoubleLinkedNode>>>,
    tail: Option<Rc<RefCell<DoubleLinkedNode>>>,
}

impl DoubleLinked {
    pub fn new(
        head: Option<Rc<RefCell<DoubleLinkedNode>>>,
        tail: Option<Rc<RefCell<DoubleLinkedNode>>>,
    ) -> Self {
        Self { head, tail }
    }

    pub fn get_head(&self) -> Option<Rc<RefCell<DoubleLinkedNode>>> {
        if self.head.is_none() {
            return None;
        }
        Some(self.head.as_ref().unwrap().clone())
    }

    pub fn get_tail(&self) -> Option<Rc<RefCell<DoubleLinkedNode>>> {
        if self.tail.is_none() {
            return None;
        }
        Some(self.tail.as_ref().unwrap().clone())
    }

    pub fn add_back(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(DoubleLinkedNode {
            key,
            value,
            prev: self.get_tail(),
            next: None,
        }));
        self.tail.replace(node);
    }

    pub fn push_front(&mut self, node: Rc<RefCell<DoubleLinkedNode>>) {
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(old_head);
                node.borrow_mut().prev = None;
                self.head = Some(node);
            }
            None => {
                // empty list: node is both head and tail
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }

    pub fn push_back(&mut self, node: Rc<RefCell<DoubleLinkedNode>>) {
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old_tail);
                node.borrow_mut().next = None;
                self.tail = Some(node);
            }
            None => {
                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    pub fn remove(&mut self, target: Rc<RefCell<DoubleLinkedNode>>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();

        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next = None;
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            (None, None) => {
                self.head.take();
                self.tail.take();
            }
        }
    }

    pub fn move_head(&mut self, target: Rc<RefCell<DoubleLinkedNode>>) {
        if !Rc::ptr_eq(self.get_head().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.push_front(target);
        }
    }

    pub fn move_tail(&mut self, target: Rc<RefCell<DoubleLinkedNode>>) {
        if !Rc::ptr_eq(self.get_tail().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.push_back(target);
        }
    }
}

struct DoubleLinkedNode {
    key: i32,
    value: i32,
    next: Option<Rc<RefCell<DoubleLinkedNode>>>,
    prev: Option<Rc<RefCell<DoubleLinkedNode>>>,
}

impl DoubleLinkedNode {
    pub fn new(key: i32, value: i32) -> Self {
        DoubleLinkedNode {
            value,
            key,
            next: None,
            prev: None,
        }
    }
}

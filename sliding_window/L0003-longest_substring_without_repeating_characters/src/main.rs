//! Leetcode: 0003-Longest Substring Without Repeating Characters
//! Link: https://leetcode.com/problems/longest-substring-without-repeating-characters
//!
//! Category: Sliding Window / Hashing
//! Level: Medium
//! Runtime: 1 ms | Beats 63.56%
//! 
//! Method of Solving: Make a sliding window.
//! While r is less than the length of the string, we advance.
//! If r is a new element not contained in the HashMap, insert, move r, and check longest.
//! If r is equal to l, then move r.
//! If r is not a new element, then remove the element at l and move l.
//! Return longest.

use std::collections::HashMap;

fn main() {
    let l = length_of_longest_substring("abcabcbb".to_string());
    println!("{:?}", l);

    let l = length_of_longest_substring("bbbbb".to_string());
    println!("{:?}", l);

    let l = length_of_longest_substring("pwwkew".to_string());
    println!("{:?}", l);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut r = 0;
    let mut l = 0;
    let mut longest = 0;
    if s.len() == 0 {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }
    let mut map = HashMap::new();
    let bytes = s.as_bytes();
    map.insert(bytes[l], l);
    while r < s.len() {
        let c1 = bytes[l];
        let c2 = bytes[r];
        if !map.contains_key(&c2) {
            map.insert(c2, r);
            r += 1;
            longest = longest.max(r - l);
        } else if r == l {
            r += 1;
        } else {
            map.remove(&c1);
            l += 1;
        }
    }

    longest as i32
}

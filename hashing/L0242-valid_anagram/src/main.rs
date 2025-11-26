//! Leetcode: 0001-Valid Anagram
//! Link: https://leetcode.com/problems/valid-anagram
//! Category: Hash Table
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%

use std::collections::HashMap;

fn main() {
    println!("{}", is_anagram("aacc".to_string(), "cacc".to_string()));
}

fn is_anagram(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    if s.len() != t.len() {
        return false;
    }

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        match map.get_mut(&c) {
            Some(v) => {
                *v -= 1;
                if *v == 0 {
                    map.remove(&c);
                }
            }
            None => return false,
        }
    }

    map.is_empty()
}

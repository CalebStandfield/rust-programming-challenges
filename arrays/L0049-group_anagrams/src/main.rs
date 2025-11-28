//! Leetcode: 0049-Group Anagrams
//! Link: https://leetcode.com/problems/group-anagrams
//! Category: arrays
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for s in strs {
        let mut frequencies = [0u8; 26];
        for c in s.bytes() {
            frequencies[(c - b'a') as usize] += 1;
        }
        map.entry(frequencies).or_insert_with(Vec::new).push(s);
    }
    map.into_values().collect()
}


//! Leetcode: 0049-Group Anagrams
//! Link: https://leetcode.com/problems/group-anagrams
//! 
//! Category: arrays
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//! 
//! Method of Solving: Use a HashMap to store the anagrams.
//! Then for each s in strs make a frequency array.
//! For every byte in s.bytes() add 1 to frequencies[c - b'a'].
//! c - b'a' is the ascii value of the character. Which corresponds to the index of the frequency array.
//! Lastly, map the entry or insert with a new vector and push the string to it.
//! After that return the values of the map. 

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


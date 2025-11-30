//! Leetcode: 0001-Valid Anagram
//! Link: https://leetcode.com/problems/valid-anagram
//! 
//! Category: Hash Table
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//! 
//! Method of Solving: Check if their sizes are the same.
//! Since we are using a map instead of an array, this time we use map.entry(c).or_insert(0) += 1.
//! Remember to dereference the map in this case.
//! Then for every character in the second string you check if the map has the character.
//! If yes, then decrement the count, if count is 0, then remove the <k, v> pair.
//! If 'None' return false. 
//! At the end return map.is_empty() since if every c in s and t have equal counts, the map should be empty.

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

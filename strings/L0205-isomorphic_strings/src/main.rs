//! Leetcode: 0205-Isomorphic Strings
//! Link: https://leetcode.com/problems/isomorphic-strings
//! Category: strings / hashmaps
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%

use std::collections::HashMap;

fn main() {
    println!("{}", is_isomorphic("egg".to_string(), "add".to_string()));
    println!("{}", is_isomorphic("foo".to_string(), "bar".to_string()));
    println!(
        "{}",
        is_isomorphic("paper".to_string(), "title".to_string())
    );
    println!("{}", is_isomorphic("badc".to_string(), "baba".to_string()));
    println!("{}", is_isomorphic(" ".to_string(), "a".to_string()));
    println!("{}", is_isomorphic("ab de".to_string(), "ed bc".to_string()));
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut to = HashMap::new();
    let mut from = HashMap::new();
    let s = s.chars();
    let t = t.chars();

    for (c1, c2) in s.zip(t) {
        // If it does contain the key
        if to.contains_key(&c1) {
            // Needs to be equal to the value. ex. e -> a
            if to[&c1] != c2 {
                return false;
            }
            // It did not contain the key
        } else {
            if from.contains_key(&c2) {
                return false;
            }
            to.insert(c1, c2);
            from.insert(c2, c1);
        }
    }
    true
}

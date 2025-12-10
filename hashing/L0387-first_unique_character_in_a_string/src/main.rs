//! Leetcode: 0387 First Unique Character in a String
//! Link: https://leetcode.com/problems/first-unique-character-in-a-string
//!
//! Category: hashing
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00
//!
//! Method of Solving: Make an array of character frequencies.
//! Update the array accordingly.
//! Then rescan the string and find the first character whose count is 1.

fn main() {
    println!("{}", first_uniq_char("loveleetcode".to_string()));
    println!("{}", first_uniq_char("leetcode".to_string()));
}

pub fn first_uniq_char(s: String) -> i32 {
    let mut char_vec = vec![0; 26];
    s.chars().for_each(|c| {
        char_vec[c as usize - 'a' as usize] += 1;
    });
    for (i, c) in s.char_indices() {
        if char_vec[c as usize - 'a' as usize] == 1 {
            return i as i32;
        }
    }
    -1
}

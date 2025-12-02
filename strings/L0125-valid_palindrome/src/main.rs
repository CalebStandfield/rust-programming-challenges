//! Leetcode: 0125 Valid Palindrome
//! Link: https://leetcode.com/problems/valid-palindrome
//!
//! Category: string
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Left and right pointers.
//! Check if the char at left is not equal to the right char.
//! If they are equal, continue and move pointers.

fn main() {
    println!("{}", is_palindrome(String::from("racecar")));
    println!(
        "{}",
        is_palindrome(String::from("A man, a plan, a canal: Panama"))
    );
    println!("{}", is_palindrome(String::from(" ")));
    println!("{}", is_palindrome(String::from("race a car")));
}

pub fn is_palindrome(s: String) -> bool {
    let s = s
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<char>>();

    if s.len() == 0 || s.len() == 1 {
        return true;
    }
    let mut left = 0;
    let mut right = s.len() - 1;

    while left <= right {
        let l = s[left];
        let r = s[right];
        if l != r {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

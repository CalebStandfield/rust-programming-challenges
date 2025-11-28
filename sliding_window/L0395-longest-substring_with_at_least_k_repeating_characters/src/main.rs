//! Leetcode: 0395-Longest Substring with At Least K Repeating Characters
//! Link: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters
//! Category: sliding window
//! Level: Medium
//! Runtime: 15 ms | Beats 20.00%
//! Note: Not happy at all with this. It's using the divide and conquer approach instead of a sliding window.
//! I don't understand the sliding window approach, so I am not going to just copy and paste someone else's solution.
//! Nor do I really want to just study the solution for the sake of knowing the best solution.
//! In this case I think it's better to move on, do other questions, and or work on my projects.

fn main() {
    println!("{}", longest_substring("aaabb".to_string(), 3));
    println!("{}", longest_substring("ababbc".to_string(), 2));
    println!("{}", longest_substring("ababacb".to_string(), 3));
}

pub fn longest_substring(s: String, k: i32) -> i32 {
    fn helper(s: &[u8], k: i32) -> i32 {
        if s.len() == 0 || k as usize > s.len() {
            return 0;
        }

        let mut freq = [0i32; 26];
        for &ch in s {
            freq[(ch - b'a') as usize] += 1;
        }

        let mut i = 0;
        while i < s.len() {
            let idx = (s[i] - b'a') as usize;
            if freq[idx] < k {
                let left = helper(&s[..i], k);
                let right = helper(&s[i + 1..], k);
                return left.max(right);
            }
            i += 1;
        }

        s.len() as i32
    }

    helper(s.as_bytes(), k)
}

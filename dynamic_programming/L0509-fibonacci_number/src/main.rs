//! Leetcode: 0409 Fibonacci Number
//! Link: https://leetcode.com/problems/fibonacci-number
//!
//! Category: dynamic programming
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Dynamic programming and recursion.
//! Base case of 0 and 1.
//! Then return fib(n - 1) + fib(n - 2).
//! Make sure to make the dp array.
//! Set the base cases inside the array.
//! Then your new base case becomes if dp[n as usize] != -1 { return dp[n as usize] }.
//! Otherwise, you need to call fib(n - 1, dp) + fib(n - 2, dp).
//! Then Remember to update dp[n as usize] to be equal to ib(n - 1, dp) + fib(n - 2, dp).
//! Return val.

fn main() {
    println!("{}", fib(10));
}

pub fn fib(n: i32) -> i32 {
    let length = n as usize;
    let mut dp = vec![-1; length as usize + 1];
    dp[0] = 0;
    if n > 0 {
        dp[1] = 1;
    }

    fn recursive(n: i32, dp: &mut Vec<i32>) -> i32 {
        if dp[n as usize] != -1 {
            return dp[n as usize]
        }

        let val = recursive(n -1, dp) + recursive(n -2, dp);
        dp[n as usize];
        val
    }

    recursive(n, &mut dp)
}

//! Leetcode: 1137 N-th Tribonacci Number
//! Link: https://leetcode.com/problems/n-th-tribonacci-number
//!
//! Category: dynamic programming
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Dynamic programming and recursion.
//! Base case of 0 and 1 and 2.
//! Then return fib(n - 1) + fib(n - 2) + fib(n - 3).
//! Make sure to make the dp array.
//! Set the base cases inside the array.
//! Then your new base case becomes if dp[n as usize] != -1 { return dp[n as usize] }.
//! Otherwise, you need to call fib(n - 1, dp) + fib(n - 2, dp) + fib(n - 3, dp).
//! Then Remember to update dp[n as usize] to be equal to ib(n - 1, dp) + fib(n - 2, dp) + fib(n - 3, dp).
//! Return val.

fn main() {
    println!("Hello, world!");
}

pub fn tribonacci(n: i32) -> i32 {
    let mut dp = vec![-1; (n + 1) as usize];
    if n == 0 {
        return 0
    }
    dp[0] = 0;
    if n == 1 {
        return 1
    }
    dp[1] = 1;
    if n == 2 {
        return 1
    }
    dp[2] = 1;
    fn helper(n: i32, dp: &mut Vec<i32>) -> i32 {
        if dp[n as usize] != -1 {
            return dp[n as usize]
        }
        let val = helper(n - 1, dp) + helper(n - 2, dp) + helper(n - 3, dp);
        dp[n as usize] = val;
        val
    }
    helper(n, &mut dp)
}
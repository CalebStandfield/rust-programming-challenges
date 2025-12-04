//! Leetcode: 0070 Climbing Stairs
//! Link: https://leetcode.com/problems/climbing-stairs
//!
//! Category: dynamic programming
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: 2 ways.
//! The first way is with a dp array.
//! It is essentially `fib` with a different starting number.
//! Another way is to keep just ways n - 1 and ways n - 2 as two i32 numbers and swap as you go up.

fn main() {
    println!("{}", climb_stairs(3));
}

pub fn climb_stairs(n: i32) -> i32 {
    let prev = 1;
    let curr = 2;
    fn helper(n: i32, mut prev: i32, mut curr: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        for _ in 3..n + 1 {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }

        curr
    }
    helper(n, prev, curr)
}

// pub fn climb_stairs(n: i32) -> i32 {
//     let mut dp = vec![-1; (n + 1) as usize];
//     dp[0] = 1;
//     if n >= 1 {
//         dp[1] = 1;
//     }
//     fn helper(n: i32, dp: &mut Vec<i32>) -> i32 {
//         if dp[n as usize] != -1 {
//             return dp[n as usize]
//         }
//         let val = helper(n - 1, dp) + helper(n - 2, dp);
//         dp[n as usize] = val;
//         val
//     }
//     helper(n, &mut dp)
// }

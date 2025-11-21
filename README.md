# 🦀 rust-algorithms

Note for a while this repo will look pretty bare as I've mostly solved LeetCode problems in other languages and will essentially be starting from scratch but this time in Rust. 

A collection of algorithm and data structure problems solved in Rust, organized by topic (graphs, dynamic programming, two pointers, etc.).  
Used to practice problem-solving, clean Rust patterns, and performance-minded coding.

## Structure

- Each top-level folder (e.g. `graphs/`, `dynamic_programming/`) groups problems by category.
- Inside each folder, every problem is its own binary crate in a Cargo workspace.
- Each crate will be named with a prefix of L, H, or another prefix depending on what site was used, like LeetCode or Hackerrank.
- Crate names follow the pattern: `L####_problem_name`.

## Problem Details
Each main.rs has //! at the top to denote problem details \
Problem details include:
- Leetcode number and name 
- Link to Leetcode problem
- Category of problem 
- Difficulty of problem
- Runtime X ms | Beats XXX.XX% 
- Notes

Since memory usage can fluctuate, I have decided to not include it in the problem details.

Example:

- `hashmaps/L0001-two_sum`
- `dynamic_programming/L0070-climbing_stairs`
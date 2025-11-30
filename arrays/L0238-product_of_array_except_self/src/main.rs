//! Leetcode: 0238-Product of Array Except Self
//! Link: https://leetcode.com/problems/product-of-array-except-self
//! 
//! Category: arrays
//! Level: medium
//! Runtime: 0 ms | Beats 100.00%
//! 
//! Method of Solving: Use two arrays to store the pre and suf products.
//! Then it's as easy as multiplying pre[i] and suf[i] together to ret[i].
//! Return ret[i].

fn main() {
    let vec = vec![1, 2, 3, 4];
    println!("{:?}", product_except_self(vec));
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pre = vec![1; n];
    let mut suf = vec![1; n];
    let mut ret = vec![1; n];

    for i in 1..n {
        pre[i] = pre[i - 1] * nums[i - 1];
    }

    for i in (0..n-1).rev() {
        suf[i] = suf[i + 1] * nums[i + 1];
    }

    for i in 0..n {
        ret[i] = pre[i] * suf[i];
    }

    ret
}

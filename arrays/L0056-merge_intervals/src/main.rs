//! Leetcode: 0056 Merge Intervals
//! Link: https://leetcode.com/problems/merge-intervals
//!
//! Category: arrays
//! Level: Medium
//! Runtime: 0 ms | Beats 100.00%
//!
//! Method of Solving: Sort the Vec of vectors by the first element.
//! Then (in place) check if the intervals can be merged.
//! Merge them and set the current vector to the interval.

fn main() {

}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() < 2 {
        return intervals;
    }

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut new_intervals: Vec<Vec<i32>> = Vec::new();
    let mut current = intervals[0].clone();


    for interval in intervals.into_iter().skip(1) {
        if interval[0] <= current[1] {
            current[1] = current[1].max(interval[1]);
        } else {
            new_intervals.push(current);
            current = interval;
        }
    }

    new_intervals.push(current);
    new_intervals
}


//! Leetcode: 0012-Roman to Integer
//! Link: https://leetcode.com/problems/roman-to-integer
//! 
//! Category: String
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%
//! 
//! Method of Solving: There is only one thing to think about, when to subtract. 
//! That is when the previous value was greater than the current character value. 
//! In that case we subtract. Otherwise, add to the sum. 

use std::cmp::{Ordering, PartialOrd};

fn main() {
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("IV".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut integer = 0;
    let mut prev = 0;
    for c in s.chars().rev() {
        let val = Roman::new(c).value();
        if val < prev {
            integer -= val;
        } else {
            integer += val;
        }
        prev = val;
    }
    integer
}

#[derive(Debug, Clone, Copy)]
enum Roman {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl Roman {
    fn value(&self) -> i32 {
        match self {
            Roman::I => 1,
            Roman::V => 5,
            Roman::X => 10,
            Roman::L => 50,
            Roman::C => 100,
            Roman::D => 500,
            Roman::M => 1000,
        }
    }

    fn new(c: char) -> Self {
        match c {
            'I' => Roman::I,
            'V' => Roman::V,
            'X' => Roman::X,
            'L' => Roman::L,
            'C' => Roman::C,
            'D' => Roman::D,
            'M' => Roman::M,
            _ => panic!("Invalid Roman character: {}", c),
        }
    }
}

impl PartialEq for Roman {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for Roman {}

impl PartialOrd for Roman {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Roman {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

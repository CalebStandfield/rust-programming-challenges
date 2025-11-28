//! Leetcode: 0020-Valid Parentheses
//! Link: https://leetcode.com/problems/valid-parentheses
//! Category: stacks
//! Level: Easy
//! Runtime: 0 ms | Beats 100.00%

fn main() {
    println!("{}", is_valid("()".to_string()));
    println!("{}", is_valid("(({{[]}}))".to_string()));
    println!("{}", is_valid("(({{[]}))".to_string()));
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' {
            if stack.is_empty() {
                return false;
            }
            let top = stack.pop().unwrap();
            if (top == '(' && c != ')') || (top == '[' && c != ']') || (top == '{' && c != '}') {
                return false;
            }
        }

    }

    stack.is_empty()
}

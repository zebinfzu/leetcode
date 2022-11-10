struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk = vec![];
        let heap: std::collections::HashMap<_, _> = [(')', '('), (']', '['), ('}', '{')].into();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stk.push(c),
                t if stk.pop().unwrap_or('*') != *heap.get(&t).unwrap_or(&'_') => {
                    return false;
                }
                _ => (),
            }
        }
        stk.len() == 0
    }
}

fn main() {
    Solution::is_valid(String::from("()"));
}

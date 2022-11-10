struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn recursion(count: (u8, u8, u8), ans: &mut Vec<String>, parentheses: &mut String) {
            match count {
                // 匹配终点
                (l, r, n) if l == r && l == n => {
                    ans.push(parentheses.to_owned());
                }
                // 匹配只能进左括号
                (l, r, n) if l == r && l < n => {
                    parentheses.push('(');
                    recursion((l + 1, r, n), ans, parentheses);
                }
                // 匹配只能进右括号
                (l, r, n) if r < n && l == n => {
                    parentheses.push(')');
                    recursion((l, r + 1, n), ans, parentheses);
                }
                // 匹配可以进左括号也可以进右括号
                (l, r, n) if l > r && l < n => {
                    // 造一个新的
                    recursion((l + 1, r, n), ans, &mut format!("{}(", parentheses));
                    // 用原来的
                    recursion((l, r + 1, n), ans, {
                        parentheses.push(')');
                        parentheses
                    });
                }
                _ => (),
            }
        }
        let mut ans: Vec<String> = vec![];
        recursion((0, 0, n as u8), &mut ans, &mut String::from(""));
        ans
    }
    pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
        match n {
            0 => vec!["".to_string()],
            1 => vec!["()".to_string()],
            n => {
                let mut ans = vec![];
                for i in 0..n {
                    let left = Self::generate_parenthesis_1(i);
                    let right = Self::generate_parenthesis_1(n - i - 1);
                    for l in left.iter() {
                        for r in right.iter() {
                            ans.push(format!("({}){}", l, r));
                        }
                    }
                }
                ans
            }
        }
    }
}
fn main() {
    println!("{:#?}", Solution::generate_parenthesis(3));
    println!("{:#?}", Solution::generate_parenthesis_1(3));
}

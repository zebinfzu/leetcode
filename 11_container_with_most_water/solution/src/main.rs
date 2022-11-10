struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut t: (usize, usize) = (0, height.len() - 1);
        let mut ans = 0;
        loop {
            t = match t {
                (l, r) if l < r => {
                    let tmp;
                    match (height[l], height[r]) {
                        (i, j) if i > j => {
                            tmp = j * (r - l) as i32;
                            t.1 -= 1;
                        }
                        (i, _) => {
                            tmp = i * (r - l) as i32;
                            t.0 += 1;
                        }
                    }
                    ans = if ans > tmp { ans } else { tmp };
                    t
                }
                _ => break,
            }
        }
        ans
    }
    #[allow(dead_code)]
    pub fn max_area_(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0 as usize, height.len() - 1);
        let mut ans = 0;
        while l < r {
            let (hl, hr) = (height.get(l).unwrap(), height.get(r).unwrap());
            let tmp;
            if hl > hr {
                tmp = hr * (r - l) as i32;
                r -= 1;
            } else {
                tmp = hl * (r - l) as i32;
                l += 1;
            }
            ans = if ans > tmp { ans } else { tmp };
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

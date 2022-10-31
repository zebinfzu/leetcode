use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        // 使用迭代器生成值和索引的元组
        for (idx, val) in nums.iter().enumerate() {
            if let Some(val) = hash.get(&(target - val)) {
                return vec![idx as i32, *val as i32];
            } else {
                hash.insert(*val, idx as i32);
            }
        }
        panic!("error")
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

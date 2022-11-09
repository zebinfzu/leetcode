use std::collections::HashMap;
#[allow(unused)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    for (idx, val) in nums.iter().enumerate() {
        // hash表当中找到值加上当前元素等于target的话，返回答案
        if let Some(val) = hash.get(&(target - val)) {
            return vec![idx as i32, *val as i32];
        } else {
            hash.insert(*val, idx as i32);
        }
    }
    panic!("error")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: Vec<i32>, target: i32, expected: (i32, i32)) {
        let ans = two_sum(nums, target);
        let get = |i: usize| -> i32 {
            match ans.get(i) {
                Some(t) => *t,
                None => -1,
            }
        };
        let ans = (get(0), get(1));
        if !((ans.0 == expected.0 && ans.1 == expected.1)
            || (ans.1 == expected.0 && ans.0 == expected.1))
        {
            panic!("error");
        }
    }

    #[test]
    fn test_1() {
        test(vec![2, 7, 11, 15], 9, (0, 1));
    }

    #[test]
    fn test_2() {
        test(vec![3, 2, 4], 6, (1, 2));
    }
}

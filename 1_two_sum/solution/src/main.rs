#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut heap: HashMap<i32, i32> = HashMap::new();
    for (idx, val) in nums.iter().enumerate() {
        if let Some(jdx) = heap.get(&(target - val)) {
            return vec![idx as i32, *jdx];
        }
        heap.insert(*val, idx as i32);
    }
    panic!("error")
}
fn main() {}

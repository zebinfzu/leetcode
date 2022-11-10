struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        for j in 1..nums.len() {
            if nums[j] != nums[j - 1] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    );
    println!("{}", Solution::remove_duplicates(&mut vec![0]));
    println!("{}", Solution::remove_duplicates(&mut vec![1, 1]));
    println!("{}", Solution::remove_duplicates(&mut vec![1, 1, 1, 1]));
    println!("{}", Solution::remove_duplicates(&mut vec![]));
}

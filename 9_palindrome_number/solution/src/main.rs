fn is_palindrome(x: i32) -> bool {
    // 利用回文数的特性，仅反转一半以避免越界
    // 处理可以被10整除的数字
    match x {
        0..=i32::MAX if x % 10 != 0 || x == 0 => {
            let mut x = x;
            let mut y = 0;
            while y < x {
                y = y * 10 + x % 10;
                x /= 10;
            }
            x == y || y / 10 == x
        }
        _ => false,
    }
}
fn main() {}

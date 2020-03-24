struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = ((x > 0) as i32) * 2 - 1;

        let mut current = x.abs();
        let mut result: i32 = 0;
        while current > 0 {
            result = match result.checked_mul(10) {
                Some(val) => val,
                None => return 0,
            };
            result = match result.checked_add(&current % 10) {
                Some(val) => val,
                None => return 0,
            };
            current /= 10;
        }
        return sign * result;
    }
}

fn main() {
    let input = 120;
    let output = Solution::reverse(input);
    println!("The solution of {} is {}", input, output)
}
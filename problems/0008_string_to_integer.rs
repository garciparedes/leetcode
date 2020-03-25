struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut result = 0;
        let mut consumed_blank = false;
        let mut sign = 1;
        for c in str.chars() {
            match c.to_digit(10) {
                Some(d) => {
                    consumed_blank = true;
                    result = match Solution::shift(result, d as i32) {
                        Some(x) => x,
                        None => return if sign > 0 { std::i32::MAX } else {std::i32::MIN }
                    };
                }
                None => {
                    if consumed_blank {
                        break;
                    }
                    match c {
                        ' ' => continue,
                        '+' => consumed_blank = true,
                        '-' => {
                            consumed_blank = true;
                            sign = -1;
                            continue;
                        }
                        _ => break,
                    }
                }
            }
        }
        return sign * result;
    }

    fn shift(result: i32, current: i32) -> Option<i32> {
        Option::Some(result)
            .and_then(|x| x.checked_mul(10))
            .and_then(|x| x.checked_add(current % 10))
    }
}

fn main() {
    let input = "-91283472332";
    let output = Solution::my_atoi(input.to_string());
    println!("The solution of \"{}\" is {}", input, output)
}
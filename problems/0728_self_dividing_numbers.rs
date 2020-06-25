use std::collections::HashSet;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for number in left..(right + 1) {
            let mut divisible = true;
            for digit in Self::decompose(number) {
                if digit == 0 || number % digit != 0 {
                    divisible = false;
                    break;
                }
            }
            if !divisible {
                continue;
            }
            result.push(number);
        }
        return result;
    }
    
    fn decompose(mut number: i32) -> HashSet<i32> {
        let mut digits = HashSet::new();
        while number != 0 {
            digits.insert(number % 10);
            number /= 10;
        }
        return digits;
    }
}

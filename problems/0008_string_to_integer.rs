struct Solution {}

fn main() {
    let input = "+-12";
    let output = Solution::my_atoi(input.to_string());
    println!("The solution of \"{}\" is {}", input, output)
}

use std::cmp::Ordering;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        Computation::compute(str)
    }
}

struct Computation {
    unsigned: i32,
    consumed_blank: bool,
    sign: i32,
}

impl Computation {
    fn compute(str: String) -> i32 {
        let mut computation = Computation::new();
        for c in str.chars() {
            if computation.process(c) {
                continue;
            };
            break;
        }
        return computation.result();
    }

    fn new() -> Self {
        Computation { unsigned: 0, consumed_blank: false, sign: 1 }
    }

    fn process(&mut self, character: char) -> bool {
        return match character.to_digit(10) {
            Some(d) => self.process_digit(d as i32),
            None => self.process_non_digit(character),
        };
    }

    fn process_digit(&mut self, digit: i32) -> bool {
        self.consumed_blank = true;
        match self.shift(digit) {
            Some(x) => {
                self.unsigned = x;
                true
            }
            None => {
                self.unsigned = self.process_overflow();
                false
            }
        }
    }
    fn process_overflow(&mut self) -> i32 {
        match self.sign.cmp(&0) {
            Ordering::Greater => std::i32::MAX,
            Ordering::Less => std::i32::MIN,
            Ordering::Equal => 0,
        }
    }

    fn shift(&mut self, current: i32) -> Option<i32> {
        Option::Some(self.unsigned)
            .and_then(|x| x.checked_mul(10))
            .and_then(|x| x.checked_add(current % 10))
    }

    fn process_non_digit(&mut self, character: char) -> bool {
        if self.consumed_blank {
            return false;
        }
        match character {
            ' ' => true,
            '+' => {
                self.consumed_blank = true;
                true
            }
            '-' => {
                self.consumed_blank = true;
                self.sign = -1;
                true
            }
            _ => false,
        }
    }

    fn result(self) -> i32 {
        return self.sign * self.unsigned;
    }
}
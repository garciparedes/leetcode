use std::cmp::Ordering;

struct Solution {}

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
        let mut computation = Computation { unsigned: 0, consumed_blank: false, sign: 1 };
        for c in str.chars() {
            if computation._process(c) {
                continue;
            };
            break;
        }
        return computation._result();
    }

    fn _process(&mut self, character: char) -> bool {
        return match character.to_digit(10) {
            Some(d) => self._process_digit(d as i32),
            None => self._process_non_digit(character),
        };
    }

    fn _process_digit(&mut self, digit: i32) -> bool {
        match self._shift(digit) {
            Some(x) => {
                self.unsigned = x;
                true
            }
            None => {
                self.unsigned = self._process_overflow();
                false
            }
        }
    }
    fn _process_overflow(&mut self) -> i32 {
        match self.sign.cmp(&0) {
            Ordering::Greater => std::i32::MAX,
            Ordering::Less => std::i32::MIN,
            Ordering::Equal => 0,
        }
    }

    fn _shift(&mut self, current: i32) -> Option<i32> {
        Option::Some(self.unsigned)
            .and_then(|x| x.checked_mul(10))
            .and_then(|x| x.checked_add(current % 10))
    }

    fn _process_non_digit(&mut self, character: char) -> bool {
        if !self.consumed_blank {
            false;
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

    fn _result(self) -> i32 {
        self.sign * self.unsigned
    }
}

fn main() {
    let input = "  -22 lasdada";
    let output = Solution::my_atoi(input.to_string());
    println!("The solution of \"{}\" is {}", input, output)
}
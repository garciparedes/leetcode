impl Solution {
    pub fn calculate(s: String) -> i32 {
        let n = s.len();
        
        let mut stack = Vec::new();
        let mut num = 0;
        let mut sign = '+';
        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                num = 10 * num + c.to_digit(10).unwrap() as i32;
            } 
            if (!c.is_numeric() && c != ' ') || i == n - 1 {
                match sign {
                    '-' => num *= -1,
                    '*' => num *= stack.pop().unwrap(),
                    '/' => num = stack.pop().unwrap() / num,
                    _ => (),
                };
                stack.push(num);
                sign = c;
                num = 0;
            }
        }
        
        return stack.into_iter().sum();
    }
}

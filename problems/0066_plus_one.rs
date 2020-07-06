impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;
        while i >= 0 && digits[i] == 9 {
            digits[i] = 0;
            if i == 0 {
                digits.insert(0, 0);
            } else {         
                i -= 1;   
            }
        }
        digits[i] += 1;   
        return digits;
    }
}

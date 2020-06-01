impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut digits: Vec<i32> = Vec::new();
        let mut current = num;
        while current != 0 {
            digits.insert(0, current % 10);
            current /= 10;
        }
        
        let mut first_six_index: i32 = -1;
        for (i, &digit) in digits.iter().enumerate() {
            if first_six_index == -1 && digit == 6 {
                first_six_index = i as i32;
                break;
            }
        }
        
        if first_six_index != -1 {
            digits[first_six_index as usize] = 9;
        }
        
        let result: i32 = digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| acc + digit * 10i32.pow(i as u32));
        return result;
    }
}

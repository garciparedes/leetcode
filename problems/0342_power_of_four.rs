impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        
        let mut one = false;
        for (i, digit) in  format!("{:b}", num).chars().rev().enumerate() {
            if digit == '1'{
                if i % 2 == 0 && !one {
                    one = true;
                } else {
                    return false;
                }
            } 
        }
        return one;
    }
}

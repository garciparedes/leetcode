impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        let mut counter = 0;
        let mut current = num;
        while current > 0 {
            if current % 2 == 1 {
                current -= 1;
            } else {
                current /= 2;
            }
            counter += 1;
        }       
        return counter;
    }
}


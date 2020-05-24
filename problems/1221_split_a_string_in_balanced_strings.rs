impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {        
        let mut counter = 0;
        let mut memory = vec![0, 0];
        for current in s.chars() {
            if current == 'L' {
                memory[0] += 1; 
            } else {
                memory[1] += 1;
            }
            if memory[0] == memory[1] {
                counter += 1;
                memory = vec![0, 0];
            }
        }
        return counter;
        
    }
}


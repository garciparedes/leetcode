impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut memory = Vec::with_capacity(n as usize);
        let mut current = 1;
        memory.push(current);

        let (mut two, mut three, mut five) = (0, 0, 0);
        for _ in 1..n {
            while two < memory.len() && current >= 2 * memory[two] {
                two += 1;
            }
            while three < memory.len() && current >= 3 * memory[three] {
                three += 1;
            }
            while five < memory.len() && current >= 5 * memory[five] {
                five += 1;
            }

            if 2 * memory[two] < 3 * memory[three] && 2 * memory[two] < 5 * memory[five] {
                current = 2 * memory[two];
            } else if 3 * memory[three] < 5 * memory[five] {
                current = 3 * memory[three];
            } else {
                current = 5 * memory[five];
            }
            memory.push(current);
        }
        return current;
    }
}


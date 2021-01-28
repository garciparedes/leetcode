use std::cmp;
use std::cmp::Ordering;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut values = vec![1; n as usize];
        let mut current = values.len() as i32;
        
        let mut i = values.len() - 1;
        for i in (0..values.len()).rev() {
            match current.cmp(&k) {
                Ordering::Less => {
                    let tmp = cmp::min(k - current, 25);
                    current += tmp;
                    values[i] += tmp as u8;
                },
                Ordering::Equal => break,
                Ordering::Greater => panic!(),
            }
        }
        
        return values
            .into_iter()
            .map(|v| ('a' as u8 + v - 1) as char)
            .collect();
    }
}

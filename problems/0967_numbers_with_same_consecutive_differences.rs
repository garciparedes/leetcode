impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {  
        if n == 1 {
            return (0..10).collect();
        }
        
        let n = n as usize;
        let k = k as i8;
        
        let mut result = Vec::new();
        for i in 1..10 {
            result.extend(Self::rec(n, k, vec![i]).into_iter());
        }
        
        return result
            .into_iter()
            .map(|raw| {
                return raw
                    .into_iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (i, v)| acc + i32::pow(10, i as u32) * v as i32);
            })
            .collect();
    }
    
    fn rec(n: usize, k: i8, current: Vec<i8>) -> Vec<Vec<i8>> {
        if current.len() == n {
            return vec![current];
        }
        let mut result = Vec::new();
        for i in 0..10 {
            if (i - current[current.len() - 1]).abs() != k {
                continue;
            }
            let mut now = current.clone();
            now.push(i);
            result.extend(Self::rec(n, k, now).into_iter());
        }
        return result;
    }
}

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut happy = Vec::new();
        while happy.len() < n as usize {
            if happy.len() % 2 == 0 {
                happy.push(0);
            } else {
                happy.push(1);
            }
        }
        loop {    
            if happy[0] > 2 {
                return String::new();
            }
            if Self::valid(&happy) {
                k -= 1;
            }
            if k == 0 {
                break;
            }
            let mut i = happy.len() - 1;
            while i > 0 && happy[i] == 2 {
                for j in i..happy.len() {
                    happy[j] = 0;
                }
                i -= 1;
            }
            happy[i] += 1;
            
        }
        
        
        return happy
            .into_iter()
            .map(|x| match x {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                _ => panic!(),
            })
            .collect();
    }
    
    fn valid(happy: &[i32]) -> bool {
        for i in 1..happy.len() {
            if happy[i] == happy[i - 1] {
                return false;
            }
        }
        return true;
    }
}


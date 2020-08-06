use std::cmp::Reverse;

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();
        
        s1.sort_unstable();
        s2.sort_unstable();
        
        return (
            (s1
                .iter()
                .zip(s2.iter()).
                all(|(&c1, &c2)| c1 <= c2)
            ) 
            || 
            (s1
                .iter()
                .zip(s2.iter()).
                all(|(&c1, &c2)| c1 >= c2)  
            )
        );
    }
}

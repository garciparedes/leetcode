use std::collections::HashSet;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let arr = [
            Self::dist(&p1, &p2), Self::dist(&p1, &p3), Self::dist(&p1, &p4), 
            Self::dist(&p2, &p3), Self::dist(&p2, &p4), Self::dist(&p3, &p4),
        ];
        let mut s: HashSet<_> = arr.into_iter().collect();
        print!("{:?}", s);
        return !s.contains(&0) && s.len() == 2;
    }
    
    fn dist(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        return (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1]);
    }
}

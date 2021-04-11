impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut friends = vec![true; n];
        
        let mut i = n - 1;
        let mut alive = n;
        while alive > 1 {
            for _ in 0..k {
                i = (i + 1) % n;
                while !friends[i] {
                    i = (i + 1) % n;
                }  
            } 
            friends[i] = false;
            alive -= 1;
        }
        while !friends[i] {
            i = (i + 1) % n;
        }  
        return (i + 1) as i32;
    }
}

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut result = Vec::new(); 
        
        for i in 0..(1 << n) {
            result.push(start ^ i ^ i >> 1);
        }
        
        return result;
    }
}

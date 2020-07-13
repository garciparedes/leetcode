impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        
        let mut count = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let mut a = arr[i];
                for k in i + 1..j {
                    a ^= arr[k];
                }                
                
                let mut b = arr[j];    
                count += (a == b) as i32;
                for k in j + 1..n {
                    b ^= arr[k];
                    count += (a == b) as i32;
                }
            }
        }
        return count;
    }
}

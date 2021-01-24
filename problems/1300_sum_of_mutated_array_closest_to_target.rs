impl Solution {
    pub fn find_best_value(mut arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        arr.sort();
        
        let mut cum = vec![0];
        for i in 0..n {
            let tmp = cum[cum.len() - 1] + arr[i];
            cum.push(tmp);
        }
        
        let mut ans = 0;
        let mut i = 0;
        let mut best = i32::max_value();
        for cur in 0..=arr[arr.len() - 1] {
            while i < arr.len() && cur >= arr[i] {
                i += 1;
            }
            let val = cum[i] + cur * (n - i) as i32;
            
            if best > (val - target).abs() {
                ans = cur;
                best = (val - target).abs();
            }
        }
        return ans;
    }
}

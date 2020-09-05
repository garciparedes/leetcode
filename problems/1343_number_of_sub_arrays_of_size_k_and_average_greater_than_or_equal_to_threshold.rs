impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let threshold = threshold * k;
        let k = k as usize;
        let mut current: i32 = arr[..k].iter().sum() ;
        let mut ans = (current >= threshold) as i32;
        for i in k..arr.len() {
            current += arr[i] - arr[i - k];
            ans += (current >= threshold) as i32;
        }
        return ans;
    }
}

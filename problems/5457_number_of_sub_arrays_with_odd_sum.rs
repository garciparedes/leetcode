impl Solution {
    pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
        let mut odds = (arr[0] % 2 == 1) as usize;
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
            odds += (arr[i] % 2 == 1) as usize;
        }
        return ((odds * (arr.len() - odds + 1)) % (1_000_000_000 + 7)) as i32;
    }
}

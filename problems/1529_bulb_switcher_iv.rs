impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let target: Vec<_> = target.chars().collect();
        let mut count = (target[0] == '1') as i32;
        for i in 1..target.len() {
            count += (target[i] != target[i - 1]) as i32;
        }
        return count;
    }
}

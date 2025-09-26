impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.into_iter().map(|task| task[0] + task[1]).min().unwrap()
    }
}

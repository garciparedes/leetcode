impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {        
        tasks.sort_unstable_by_key(|task| task[1] - task[0]);
        
        let mut curr = 0;
        for task in tasks {
            curr += task[0];
            if curr < task[1] {
                curr += (task[1] - curr);
            }
        }
        
        return curr;
    }
}

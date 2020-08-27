impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        
        let mut result = Vec::new();
        for i in 0..intervals.len() {
            let mut index = -1;
            for j in 0..intervals.len() {
                if i == j {
                    continue;
                }
                if intervals[i][1] > intervals[j][0] {
                    continue;
                }
                if index >= 0 && intervals[j][0] > intervals[index as usize][0] {
                    continue;
                }
                
                index = j as i32;
            }
            result.push(index);
        }
        return result;
    }
}

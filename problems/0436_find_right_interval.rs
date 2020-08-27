impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut aux: Vec<(i32, usize)> = intervals
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v[0], i))
            .collect();
        aux.sort_unstable();
        
        let mut result = Vec::new();
        for i in 0..intervals.len() {
            let mut index = -1;
            for (v, j) in aux.iter() {
                if i == *j {
                    continue;
                }
                if intervals[i][1] > *v {
                    continue;
                }
                index = *j as i32;
                break;
            }
            result.push(index);
        }
        return result;
    }
}

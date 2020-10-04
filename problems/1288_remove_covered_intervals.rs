impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| (x[0], -x[1]));
        let mut i = 0;
        while i < intervals.len() {
            let mut j = i + 1;
            while j < intervals.len() {
                if !Self::covered(&intervals[i], &intervals[j]) {
                    j += 1;
                    continue;
                }
                intervals.remove(j);
            }
            i += 1;
        }
        return intervals.len() as i32;
    }
    
    fn covered(first: &Vec<i32>, second: &Vec<i32>) -> bool {
        first[0] <= second[0] && second[1] <= first[1]
    }
}

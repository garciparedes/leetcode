use std::cmp;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        points.sort_unstable();
        
        let mut ans = 0;
        let (mut i, mut j) = (0, 1);
        while i < n {
            let mut current = points[i].clone();
            while j < n && Self::overlap(&current, &points[j]) {
                current = vec![
                    cmp::max(current[0], points[j][0]), cmp::min(current[1], points[j][1])
                ];
                j += 1;
            }
            ans += 1;
            i = j;
            j += 1
        }
        return ans;
        
    }
    
    fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[0] <= b[1] && b[0] <= a[1]
    }
}

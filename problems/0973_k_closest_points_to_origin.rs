impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ans: Vec<Vec<i32>> = points[..k].iter().cloned().collect();
        ans.sort_unstable_by_key(|point| Self::squared_euclidean_norm(&point));
        
        for point in points[k..].iter() {
            let mut i = ans.len();
            while i > 0 && Self::squared_euclidean_norm(&point) < Self::squared_euclidean_norm(&ans[i - 1]) {
                i -= 1;
            }
            
            if i == ans.len() {
                continue;
            }
            
            ans.insert(i, point.to_vec());
            ans.remove(ans.len() - 1);
        }
        return ans;
    }
    
    fn squared_euclidean_norm(point: &Vec<i32>) -> i32 {
        i32::pow(point[0].abs(), 2) + i32::pow(point[1].abs(), 2)
    }
}

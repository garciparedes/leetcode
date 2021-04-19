impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|q| Self::helper(q[0], q[1], q[2] as f64, &points))
            .collect()
    }
    
    fn helper(x: i32, y: i32, r: f64, points: &[Vec<i32>]) -> i32 {        
        points
            .into_iter()
            .filter(|point| Self::euclidean_distance(x, y, point[0], point[1]) <= r)
            .count() as i32
    }
    
    fn euclidean_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
    }
}

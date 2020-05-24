impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut step_count = 0;

        let mut current = points[0].clone();
        for (i, point) in points.iter().enumerate() {
            if i == 0 {
                continue;
            }    

            while current[0] != point[0] || current[1] != point[1] {
                current[0] += if current[0] < point[0] { 1 } else if current[0] == point[0] { 0 } else { -1 };
                current[1] += if current[1] < point[1] { 1 } else if current[1] == point[1] { 0 } else { -1 };
                step_count += 1;
            }

        }      

        return step_count;
    }
}

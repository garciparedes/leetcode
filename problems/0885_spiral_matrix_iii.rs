impl Solution {
    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {        
        let mut directions_iterator = [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().cycle();
        let mut direction = &(0, 0);
        let mut max_repetitions = 0;
        let mut current_repetitions = 0;
        
        let mut current = vec![r0, c0];
        let mut result = Vec::new();
        while result.len() < (r * c) as usize {
            if 0 <= current[0] && current[0] < r && 0 <= current[1] && current[1] < c {
                result.push(current.clone());
            }
            if max_repetitions == current_repetitions {
                current_repetitions = 0;
                direction = directions_iterator.next().unwrap();
                if direction == &(0, 1) || direction == &(0, -1) {
                    max_repetitions += 1;   
                }
            }
            current[0] += direction.0;
            current[1] += direction.1;
            current_repetitions += 1;
        }
        return result;
    }
}

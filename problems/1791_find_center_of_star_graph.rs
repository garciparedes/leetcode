impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; edges.len() + 1];
        
        for edge in edges {
            if seen[edge[0] as usize - 1] {
                return edge[0];
            }
            if seen[edge[1] as usize - 1] {
                return edge[1];
            }
            seen[edge[0] as usize - 1] = true;
            seen[edge[1] as usize - 1] = true;
        }
        return -1;
    }
}

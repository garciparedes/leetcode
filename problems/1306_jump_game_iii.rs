impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        return Self::dfs(&arr, start as usize, &mut visited);
    }
    
    fn dfs(arr: &[i32], i: usize, visited: &mut[bool]) -> bool {
        if arr[i] == 0 {
            return true;
        }
        
        if visited[i] {
            return false;
        }
        visited[i] = true;
        
        
        if (i + arr[i] as usize) < arr.len() && Self::dfs(arr, i + arr[i] as usize, visited) {
            return true;
        }
        
        if i >= arr[i] as usize && Self::dfs(arr, i - arr[i] as usize, visited) {
            return true;
        }
        
        return false;
        
    }
}

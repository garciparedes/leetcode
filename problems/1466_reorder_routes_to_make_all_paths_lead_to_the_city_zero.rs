use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut adjacency = HashMap::new();
        for edge in connections {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adjacency.entry(a).or_insert_with(HashSet::new).insert((b, true));
            adjacency.entry(b).or_insert_with(HashSet::new).insert((a, false));
        }
        let mut visited = vec![false; adjacency.len()];
        visited[0] = true;
        
        return Self::dfs(&adjacency, &mut visited, 0);
    }
    
    fn dfs(adjacency: &HashMap<usize, HashSet<(usize, bool)>>, visited: &mut [bool], current: usize) -> i32 {
        let mut ans = 0;
        if let Some(children) = adjacency.get(&current) {
            for (next, direction) in children {
                if visited[*next] {
                    continue;
                }
                visited[*next] = true;

                ans += Self::dfs(adjacency, visited, *next);
                if *direction {
                    ans += 1;
                }
            }
        }
        return ans;
        
    }
}

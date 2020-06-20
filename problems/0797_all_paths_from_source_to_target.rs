impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return Solution::navigate(&graph, 0, (graph.len() - 1) as i32);
    }
    
    
    fn navigate(graph: &Vec<Vec<i32>>, origin: i32, target: i32) -> Vec<Vec<i32>> {
        if origin == target {
            return vec![vec![target]];
        }
        
        return graph[origin as usize]
            .iter()
            .map(|&destination| {
                return Solution::navigate(graph, destination, target)
                    .into_iter()
                    .map(|mut path| {
                        path.insert(0, origin);
                        return path;
                    });
            })
            .flatten()
            .collect();
    }
}

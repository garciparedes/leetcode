use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        
        let mut n = n as usize;
        let mut adjacency = vec![HashSet::new(); n];
        for edge in edges {
            adjacency[edge[0] as usize].insert(edge[1] as usize);
            adjacency[edge[1] as usize].insert(edge[0] as usize);
        }
        
        let mut leaves = Vec::new();
        for i in 0..n {
            if adjacency[i].len() == 1 {
                leaves.push(i)
            }
        }
        while n > 2 {
            n -= leaves.len();
            let mut tmp = Vec::new();
            for i in leaves {
                let j = *adjacency[i].iter().next().unwrap();
                adjacency[j].remove(&i);
                if adjacency[j].len() == 1 {
                    tmp.push(j);
                }
            }
            leaves = tmp;
        }
        
        return leaves
            .into_iter()
            .map(|i| i as i32)
            .collect();
    }
}

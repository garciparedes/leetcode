use std::cmp; 

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        
        let mut adj = vec![vec![9999; n]; n];
        for edge in edges {
            adj[edge[0] as usize - 1][edge[1] as usize - 1] = 1;
            adj[edge[1] as usize - 1][edge[0] as usize - 1] = 1;
        }
        
        for k in  0..n {
            for i in 0..n {
                for j in 0..n {
                    adj[i][j] = cmp::min(adj[i][j], adj[i][k] + adj[k][j]);
                }
            }
        }        
        
        let mut ans = vec![0; n - 1];
        let mut current = Vec::new();
        for i in 0..n {
            Self::combinations(&mut current, i, n, &mut ans, &adj);
        }
        return ans;
    }
    
    fn combinations(current: &mut Vec<usize>, i: usize, n: usize, ans: &mut Vec<i32>, adj: &Vec<Vec<usize>>) {
        if i == n {
            return;
        }
        current.push(i);
        if current.len() > 1 {
            Self::process(current, ans, adj);
        }
        for j in i + 1..n {
            Self::combinations(current, j, n, ans, adj);
        }
        current.pop();
    }
    
    fn process(current: &Vec<usize>, ans: &mut Vec<i32>, adj: &Vec<Vec<usize>>) {
        let n = current.len();
        let (mut steps, mut distance) = (0, 0);
        for i in 0..n {
            for j in i + 1..n {
                if adj[current[i]][current[j]] == 1 {
                    steps += 1
                }
                distance = cmp::max(distance,  adj[current[i]][current[j]]);
            }
        }
        if steps == n - 1 {
            ans[distance - 1] += 1;
        }
    }
}

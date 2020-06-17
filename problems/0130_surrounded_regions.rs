use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() == 0 {
            return;
        }
        
        let (n, m) = (board.len(), board[0].len());
        let mut borders: Vec<(usize, usize)> = Vec::new();
        let mut graph: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell != 'O' {
                    continue;
                }
                
                if i == 0 || i == n - 1 || j == 0 || j == m -1 {
                    borders.push((i, j));
                }
                
                let edges = graph.entry((i, j)).or_insert(HashSet::new());
                if i > 0 && board[i - 1][j] == 'O' {
                    (*edges).insert((i - 1, j));
                }
                if i < n - 1 && board[i + 1][j] == 'O' {
                    (*edges).insert((i + 1, j));
                }
                if j > 0 && board[i][j - 1] == 'O' {
                    (*edges).insert((i, j - 1));
                }
                if j < m - 1 && board[i][j + 1] == 'O' {
                    (*edges).insert((i, j + 1));
                }
            }
        }
        
        for border in borders {
            Solution::prune(&mut graph, border);
        }
        
        for node in graph.keys() {
            board[node.0][node.1] = 'X';
        }
    }
    
    fn prune(graph: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>, node: (usize, usize)) {
        match graph.remove_entry(&node) {
            Some((_, edges)) => {
                for edge in edges {
                    Solution::prune(graph, edge);
                }
            },
            _ => (),
        }
        
    }
}

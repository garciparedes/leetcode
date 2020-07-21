use std::collections::HashSet;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let (n, m) = (board.len(), board[0].len());
        for i in 0..n {
            for j in 0..m {
                if Self::dfs(&board, i, j, HashSet::new(), &word.as_slice()) {
                    return true;
                }
            }
        }
        return false;
    }
    
    fn dfs(
        board: &Vec<Vec<char>>, 
        i:usize, 
        j:usize, 
        mut visited: HashSet<(usize, usize)>, 
        word: &[char]
    ) -> bool {
        if word.len() == 0 {
            return true;
        }
        if i < 0 || board.len() <= i || j < 0 || board[0].len() <= j {
            return false;
        }
        if !visited.insert((i, j)) {
            return false;
        }
        if board[i][j] != word[0] {
            return false;
        }
        return (
            Self::dfs(&board, i + 1, j, visited.clone(), &word[1..])
            || Self::dfs(&board, i - 1, j, visited.clone(), &word[1..])
            || Self::dfs(&board, i, j + 1, visited.clone(), &word[1..])
            || Self::dfs(&board, i, j - 1, visited, &word[1..])
        );
    }
}

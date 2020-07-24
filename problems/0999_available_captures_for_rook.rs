impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (n, m) = (board.len(), board[0].len());
        let rock = Self::rock_index(&board);
        
        let mut count = 0;
        for i in rock.0 + 1..n {
            if board[i][rock.1] == '.' {
                continue;
            } 
            count += (board[i][rock.1] == 'p') as i32;
            break;
        }
        
        for i in (0..rock.0).rev() {
            if board[i][rock.1] == '.' {
                continue;
            } 
            count += (board[i][rock.1] == 'p') as i32;
            break;
        }
        
        for j in rock.1 + 1..m {
            if board[rock.0][j] == '.' {
                continue;
            } 
            count += (board[rock.0][j] == 'p') as i32;
            break;
        }
        
        for j in (0..rock.1).rev() {
            if board[rock.0][j] == '.' {
                continue;
            } 
            count += (board[rock.0][j] == 'p') as i32;
            break;
        }
        
        return count;
    }
    
    fn rock_index(board: &Vec<Vec<char>>) -> (usize, usize) {
        let (n, m) = (board.len(), board[0].len());
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'R' {
                    return (i, j);
                }
             }
        }
        panic!();
    }
}

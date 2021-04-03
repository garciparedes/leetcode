use std::char;

static MOVEMENTS: &[(isize, isize)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        Self::reveal(&mut board, click[0] as usize, click[1] as usize);
        return board;
    }
    
    
    fn reveal(board: &mut Vec<Vec<char>>, x: usize , y: usize) {
        let (n, m) = (board.len(), board[0].len());
        match board[x][y] {
            'M' => board[x][y] = 'X',
            'E' => {
                let (mut mines, mut unrevealeds) = (0, Vec::new());
                for &(i, j) in MOVEMENTS {
                    let (xx, yy) = (x as isize + i, y as isize + j);
                    if xx < 0 || xx >= n as isize || yy < 0 || yy >= m as isize {
                        continue;
                    }
                    let (xx, yy) = (xx as usize, yy as usize);
                    if board[xx][yy] == 'M' {
                        mines += 1;
                    } else if board[xx][yy] == 'E' {
                        unrevealeds.push((xx, yy));
                    }
                }
                if mines == 0 {
                    board[x][y] = 'B';
                    for (xx, yy) in unrevealeds {
                        Self::reveal(board, xx, yy);
                    }
                } else {
                    board[x][y] = char::from_digit(mines, 10).unwrap();
                }
            },
            _ => (),
        }
    }
    
}

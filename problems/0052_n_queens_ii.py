class Solution:
    def totalNQueens(self, n: int) -> int:
        board = [[False for _ in range(n)] for _ in range(n)]
        
        ans = self._traverse(board, 0)
        return ans
    
    def _traverse(self, board: list[list[bool]], i: int) -> int:
        if i >= len(board):
            return 1
        
        ans = 0
        for j in range(len(board[0])):
            if not self._is_valid(board, i, j):
                continue
                
            board[i][j] = True
            ans += self._traverse(board, i + 1)
            board[i][j] = False
            
        return ans
    
    
    @staticmethod
    def _is_valid(board: list[list[bool]], i: int, j: int) -> bool:
        for k in range(0, i):
            if board[k][j]:
                return False
        
        for k in range(len(board[0])):
            if board[i][k]:
                return False
        
        for k in range(min(i, j)):
            if board[i - (k + 1)][j - (k + 1)]:
                return False
        
        for k in range(min(i, ((len(board[0]) - 1) - j))):
            if board[i - (k + 1)][j + (k + 1)]:
                return False
            
        return True

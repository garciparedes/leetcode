class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        for i in range(9):
            if not self._valid_row(board, i):
                return False
            
            if not self._valid_column(board, i):
                return False
            
            
        for i in range(3):
            for j in range(3):
                if not self._valid_square(board, i, j):
                    return False
            
        return True
            
    def _valid_row(self, board: List[List[str]], index: int) -> bool:
        seen = set()
        for i in range(9):
            if board[index][i] == ".":
                continue
                
            if board[index][i] in seen:
                return False
            
            seen.add(board[index][i])
            
        return True
            
    def _valid_column(self, board: List[List[str]], index: int) -> bool:
        seen = set()
        for i in range(9):
            if board[i][index] == ".":
                continue
                
            if board[i][index] in seen:
                return False
            
            seen.add(board[i][index])
            
        return True
            
    def _valid_square(self, board: List[List[str]], row_index: int, column_index: int) -> bool:
        seen = set()
        for i in range(3):
            for j in range(3):
                if board[row_index * 3 + i][column_index * 3 + j] == ".":
                    continue

                if board[row_index * 3 + i][column_index * 3 + j] in seen:
                    return False

                seen.add(board[row_index * 3 + i][column_index * 3 + j])
            
        return True

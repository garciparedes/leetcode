from copy import deepcopy


class Solution:
    def gameOfLife(self, board: List[List[int]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        previous = deepcopy(board)
        
        for i in range(len(board)):
            for j in range(len(board[0])):
                count = self._count_neighbors(previous, i, j)
                if previous[i][j] and (count < 2 or count > 3):
                    board[i][j] = 0
                if not previous[i][j] and count == 3:
                    board[i][j] = 1
                
    @staticmethod
    def _count_neighbors(board: List[List[int]], i: int, j: int) -> int:
        count = 0
        if i > 0 and j > 0 and board[i - 1][j - 1]:
            count += 1
        if i > 0 and board[i - 1][j]:
            count += 1
        if i > 0 and j < len(board[0]) - 1 and board[i - 1][j + 1]:
            count += 1
        if j > 0 and board[i][j - 1]:
            count += 1
        if j < len(board[0]) - 1 and board[i][j + 1]:
            count += 1
        if i < len(board) - 1 and j > 0 and board[i + 1][j - 1]:
            count += 1
        if i < len(board) - 1 and board[i + 1][j]:
            count += 1
        if i < len(board) - 1 and j < len(board[0]) - 1 and board[i + 1][j + 1]:
            count += 1
        return count

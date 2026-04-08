class Solution:
    def minimumArea(self, grid: List[List[int]]) -> int:
        topmost, bottonmost = len(grid), -1
        leftmost, rightmost = len(grid[0]), -1
        for i, row in enumerate(grid):
            for j, cell in enumerate(row):
                if cell == 1:
                    topmost, bottonmost = min(topmost, i), max(bottonmost, i)
                    leftmost, rightmost = min(leftmost, j), max(rightmost, j)
        return (1 + abs(topmost - bottonmost)) * (1 + abs(leftmost - rightmost))

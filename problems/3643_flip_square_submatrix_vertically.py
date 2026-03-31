class Solution:
    def reverseSubmatrix(self, grid: List[List[int]], x: int, y: int, k: int) -> List[List[int]]:
        for i in range(x, x +  k // 2):
            for j in range(y, y + k):
                ii = k + 2 * x - i - 1
                grid[i][j], grid[ii][j] = grid[ii][j], grid[i][j]
        return grid

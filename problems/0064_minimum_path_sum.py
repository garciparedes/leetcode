class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        
        for j in range(1, len(grid[0])):
            grid[0][j] += grid[0][j - 1]
        
        for i in range(1, len(grid)):
            grid[i][0] += grid[i - 1][0]
            for j in range(1, len(grid[0])):
                grid[i][j] += min(grid[i][j - 1], grid[i - 1][j])
                
        return grid[len(grid) - 1][len(grid[0]) - 1]

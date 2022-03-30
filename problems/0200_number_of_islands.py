class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        ans = 0
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if self._traverse(grid, i, j):
                    ans += 1
        return ans
        
    def _traverse(self, grid: List[List[str]], i: int, j: int) -> bool:
        if grid[i][j] != "1":
            return False
        
        grid[i][j] = "2"
        
        if i > 0:
            self._traverse(grid, i - 1, j)
        if i < len(grid) - 1:
            self._traverse(grid, i + 1, j)        
        if j > 0:
            self._traverse(grid, i, j - 1)
        if j < len(grid[0]) - 1:
            self._traverse(grid, i, j + 1)
        
        return True

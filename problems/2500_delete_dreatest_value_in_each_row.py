class Solution:
    def deleteGreatestValue(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])

        for row in grid:
            row.sort(reverse=True)
        
        ans = 0
        for j in range(m):
            curr = 0
            for i in range(n):
                curr = max(curr, grid[i][j])
            ans += curr
        return ans

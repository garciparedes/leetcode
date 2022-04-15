class Solution:
    def findBall(self, grid: List[List[int]]) -> List[int]:
        ans = list()
        for k in range(len(grid[0])):
            j = k
            
            for i in range(len(grid)):
                side = grid[i][j]
                if not (0 <= j + side < len(grid[0])) or grid[i][j + side] != side:
                    j = -1
                    break
                j += side
            
            ans.append(j)
            
        return ans

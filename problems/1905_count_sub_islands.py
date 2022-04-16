class Solution:
    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        ans = 0
        for i in range(len(grid2)):
            for j in range(len(grid2[0])):
                if self._is_island(grid1, grid2, i, j) == 1:
                    ans += 1
        return ans
                
                
    def _is_island(self, grid1, grid2, i, j) -> int:
        if i < 0 or i >= len(grid2) or j < 0 or j >= len(grid2[0]) or grid2[i][j] != 1:
            return 0
        
        grid2[i][j] = 2
        
        return max(
            [
                (1 if grid1[i][j] else 2),
                self._is_island(grid1, grid2, i - 1, j),
                self._is_island(grid1, grid2, i + 1, j),
                self._is_island(grid1, grid2, i, j - 1),
                self._is_island(grid1, grid2, i, j + 1),
            ]
        )

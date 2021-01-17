"""
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight
"""

class Solution:
    def construct(self, grid: List[List[int]]) -> 'Node':
        n = len(grid)
        zeros = self.count_zeros(grid)
        if zeros == n * n:
            return Node(0, True, None, None, None, None)
        if zeros == 0:
            return Node(1, True, None, None, None, None)
        
        grid_00, grid_01, grid_10, grid_11 = self.count_subparts(grid)
        
        top_left = self.construct(grid_00)
        top_right = self.construct(grid_01)
        bottom_left = self.construct(grid_10)
        bottom_right = self.construct(grid_11)
        
        return Node(0, False, top_left, top_right, bottom_left, bottom_right)

        
    
    def count_subparts(
        self,
        grid: List[List[int]]
      ) -> Tuple[List[List[int]], List[List[int]], List[List[int]], List[List[int]]]:
        n = len(grid)
        n_2 = n // 2
        
        grid_00 = [[0 for _ in range(n_2)] for _ in range(n_2)]
        grid_01 = [[0 for _ in range(n_2)] for _ in range(n_2)]
        grid_10 = [[0 for _ in range(n_2)] for _ in range(n_2)]
        grid_11 = [[0 for _ in range(n_2)] for _ in range(n_2)]
        
        for i, row in enumerate(grid):
            for j, cell in enumerate(row):
                if cell == 0:
                    continue
                if i < n / 2:
                    if j < n / 2:
                        grid_00[i][j] = 1
                    else:
                        grid_01[i][j - n_2] = 1
                else:
                    if j < n / 2:
                        grid_10[i - n_2][j] = 1
                    else:
                        grid_11[i - n_2][j - n_2] = 1
        return grid_00, grid_01, grid_10, grid_11
        
        
        
    def count_zeros(self, grid: List[List[int]]) -> int:
        zeros, ones = 0, 0
        for row in grid:
            for cell in row:
                if cell == 0:
                    zeros += 1
        return zeros

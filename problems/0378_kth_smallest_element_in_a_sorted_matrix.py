from heapq import heappush, heappop


class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        heap = list()
        
        for i in range(len(matrix)):
            heappush(heap, (matrix[i][0], i, 0))
            
        for _ in range(k - 1):
            _, i, j = heappop(heap)
            
            if j < len(matrix) - 1:
                heappush(heap, (matrix[i][j + 1], i, j + 1))
            
        ans, _, _ = heappop(heap)
        
        return ans

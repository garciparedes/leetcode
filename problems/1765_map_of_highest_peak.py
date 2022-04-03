from heapq import heappop, heappush

class Solution:
    def highestPeak(self, isWater: List[List[int]]) -> List[List[int]]:
        heap = list()
        
        height = list()
        for i in range(len(isWater)):
            tmp = list()
            for j in range(len(isWater[0])):
                if isWater[i][j] == 1:
                    heappush(heap, (0, i, j))
                tmp.append(-1)
            height.append(tmp)
        
        while len(heap):
            h, i, j = heappop(heap)
        
            if height[i][j] != -1:
                continue
                
            height[i][j] = h
                
            if i > 0 and height[i - 1][j] == -1:
                heappush(heap, (h + 1, i - 1, j))
            
            if i < len(height) - 1 and height[i + 1][j] == -1:
                heappush(heap, (h + 1, i + 1, j))
            
            if j > 0 and height[i][j - 1] == -1:
                heappush(heap, (h + 1, i, j - 1))
                
            if j < len(height[0]) - 1 and height[i][j + 1] == -1:
                heappush(heap, (h + 1, i, j + 1))
        
        
        return height
        
        

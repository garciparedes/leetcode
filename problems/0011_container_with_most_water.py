class Solution:
    def maxArea(self, height: List[int]) -> int:
        i = 0
        j = len(height) - 1

        best = self.compute(height, i, j)
        
        while i < j:
            if height[i] < height[j]:
                i += 1
            else:
                j -= 1

            c1 = self.compute(height, i, j)
            
            if best < c1:
                best = c1
            
        return best
        
        
    def compute(self, height, i, j):
        l = height[i]
        r = height[j]
        
        h = min(l, r)
        w = j - i

        c = w * h
        return c

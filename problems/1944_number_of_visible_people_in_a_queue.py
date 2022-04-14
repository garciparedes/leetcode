class Solution:
    def canSeePersonsCount(self, heights: List[int]) -> List[int]:
        ans = [0 for _ in range(len(heights))]
        
        stack = list()
        for i, v in enumerate(heights):
            while len(stack) and heights[stack[-1]] <= v:
                ans[stack.pop()] += 1
            if len(stack):
                ans[stack[-1]] += 1
            stack.append(i)
        
        return ans

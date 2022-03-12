class Solution:
    def targetIndices(self, nums: List[int], target: int) -> List[int]:
        nums.sort()
        
        ans = list()
        for i, num in enumerate(nums):
            if num == target:
                ans.append(i)
            elif num > target:
                break
        
        return ans

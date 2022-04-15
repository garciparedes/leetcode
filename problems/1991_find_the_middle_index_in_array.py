class Solution:
    def findMiddleIndex(self, nums: List[int]) -> int:
        total = sum(nums)
        current = 0
        for i in range(len(nums)):
            current += nums[i]
            if current - nums[i] == total - current:
                return i
        return -1

class Solution:
    def countPartitions(self, nums: List[int]) -> int:
        total = sum(nums)
        current = 0
        count = 0
        for num in nums[:-1]:
            current += num
            if ((total - current) - current) % 2 == 0:
                count += 1
        return count
        
        

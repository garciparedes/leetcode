class Solution:
    def triangularSum(self, nums: List[int]) -> int:
        for _ in range(len(nums) - 1):
            curr = list()
            for i in range(len(nums) - 1):
                curr.append((nums[i] + nums[i + 1]) % 10)
            nums = curr
        return nums[0]

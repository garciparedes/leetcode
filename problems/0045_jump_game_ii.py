class Solution:
    def jump(self, nums: List[int]) -> int:
        ans = 0
        end = 0
        longest = 0
        for i in range(len(nums) - 1):
            longest = max(longest, i + nums[i])
            if i == end:
                ans += 1
                end = longest
        return ans

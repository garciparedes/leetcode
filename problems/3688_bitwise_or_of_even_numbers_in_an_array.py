class Solution:
    def evenNumberBitwiseORs(self, nums: List[int]) -> int:
        ans = 0
        for num in nums:
            if num % 2 == 0:
                ans |= num
        return ans

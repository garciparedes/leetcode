class Solution:
    def duplicateNumbersXOR(self, nums: List[int]) -> int:
        seen = set()
        ans = 0
        for num in nums:
            if num in seen:
                ans ^= num
            seen.add(num)
        return ans

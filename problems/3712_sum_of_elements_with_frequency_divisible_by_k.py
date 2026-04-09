from collections import Counter

class Solution:
    def sumDivisibleByK(self, nums: List[int], k: int) -> int:
        counter = Counter(nums)
        ans = 0
        for num, count in counter.items():
            if count % k == 0:
                ans += num * count
        return ans

from itertools import combinations
from functools import reduce


class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        ans = 0
        for i in range(1, len(nums) + 1):
            for subset in combinations(nums, i):
                ans += reduce(lambda a, b: a ^ b, subset)
        return ans  

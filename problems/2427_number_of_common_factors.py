class Solution:
    def commonFactors(self, a: int, b: int) -> int:
        return len(self.common_factors(a) & self.common_factors(b))

    def common_factors(self, num: int) -> set[int]:
        ans = set()
        ans.add(num)
        for curr in range(1, num // 2 + 1):
            if (num % curr == 0):
                ans.add(curr)
        return ans

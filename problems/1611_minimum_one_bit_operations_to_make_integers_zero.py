class Solution:
    def minimumOneBitOperations(self, n: int) -> int:
        ans = 0
        sign = 1
        for i in reversed(range(32)):
            if (n & 2 ** i) != 0:
                ans += sign * (2 ** (i + 1) - 1)
                sign *= -1
        return ans

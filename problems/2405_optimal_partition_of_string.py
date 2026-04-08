class Solution:
    def partitionString(self, s: str) -> int:
        unique = set()
        ans = 0
        for c in s:
            if c in unique:
                ans += 1
                unique = set()
            unique.add(c)
        ans += 1
        return ans

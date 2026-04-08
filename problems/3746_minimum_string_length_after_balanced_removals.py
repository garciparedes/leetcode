class Solution:
    def minLengthAfterRemovals(self, s: str) -> int:
        a_count = 0
        for c in s:
            if c == "a":
                a_count += 1
        return abs(len(s) - 2 * a_count)

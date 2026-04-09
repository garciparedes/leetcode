class Solution:
    def minLength(self, s: str) -> int:
        previous = None
        while s != previous:
            previous = s
            s = previous.replace("AB", "").replace("CD", "")
        return len(s)

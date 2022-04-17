class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        consecutives = list()
        current = s[0]
        count = 0
        for c in s:
            if c == current:
                count += 1
            else:
                current = c
                consecutives.append(count)
                count = 1
        consecutives.append(count)
        
        ans = 0
        for i in range(len(consecutives) - 1):
            ans += min(consecutives[i], consecutives[i + 1])
        
        return ans

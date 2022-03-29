class Solution:
    def firstUniqChar(self, s: str) -> int:
        seen = dict()
        for i, c in enumerate(s):
            if c in seen:
                seen[c] = None
            else:   
                seen[c] = i
                
        ans = len(s)
        for i in seen.values():
            if i is not None:
                ans = min(ans, i)
                
        if ans == len(s):
            ans = -1
        
        return ans

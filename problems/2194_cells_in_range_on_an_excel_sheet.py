class Solution:
    def cellsInRange(self, s: str) -> List[str]:
        y1, x1 = s[0], s[1]
        y2, x2 = s[3], s[4]
        
        ans = list()
        for i in range(ord(y2) - ord(y1) + 1):
            for j in range(ord(x2) - ord(x1) + 1):
                ans.append(f"{chr(ord(y1) + i)}{chr(ord(x1) + j)}")
        return ans

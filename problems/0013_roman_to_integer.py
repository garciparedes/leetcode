class Solution:
    MAPPER = {
        "": 0,
        "I": 1,
        "V": 5,
        "X": 10,
        "L": 50,
        "C": 100,
        "D": 500,
        "M": 1000
    }
        
    def romanToInt(self, s: str) -> int:
        ans = 0
        current = ""
        count = 0
        for r in s:
            if current == r:
                count += 1
            else:
                val = self.MAPPER[current] * count
                if self.MAPPER[r] > self.MAPPER[current]:
                    val *= -1
                ans += val
                current = r
                count = 1
                
        ans += self.MAPPER[current] * count
                
        return ans

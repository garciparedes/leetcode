from itertools import zip_longest


class Solution:
    MAPPER = {
        1000: ["M", "MM", "MMM"],
        100: ["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
        10: ["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
        1: ["I", "II", "III","IV", "V", "VI", "VII", "VIII", "IX",],
    }
    
    def intToRoman(self, num: int) -> str:
        ans = str()
        for k, v in self.MAPPER.items():
            if num >= k:
                ans += v[num // k - 1]
                num = num % k
        return ans

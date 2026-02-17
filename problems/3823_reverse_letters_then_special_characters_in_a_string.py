import string

class Solution:
    def reverseByType(self, s: str) -> str:
        letters = list()
        symbols = list()
        for c in s: 
            if c in string.ascii_lowercase:
                letters.append(c)
            else:
                symbols.append(c)

        ans = ""
        for c in s:
            if c in string.ascii_lowercase:
                ans += letters.pop()
            else:
                ans += symbols.pop()
        return ans

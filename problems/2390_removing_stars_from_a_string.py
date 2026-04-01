class Solution:
    def removeStars(self, s: str) -> str:
        chars = list() 

        for c in s:
            if (c == "*"): 
                chars.pop()
            else:
                chars.append(c)

        return "".join(chars)

class Solution:
    def countAndSay(self, n: int) -> str:
        return self._count_and_say(n)
        
        
    def _count_and_say(self, n: int) -> str:
        if n == 1:
            return "1"
        
        word = self._count_and_say(n - 1)
        
        ans = str()
        
        curr = word[0]
        count = 1
        for digit in word[1:]:
            if digit == curr:
                count += 1
            else:
                ans += f"{count}{curr}"
                curr = digit
                count = 1
                
        if count:
            ans += f"{count}{curr}"

        return ans

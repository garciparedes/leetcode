class Solution:
    def minSwaps(self, s: str) -> int:
        counter = 0
        
        ans = 0
        for c in s:
            if c == "[":
                counter += 1
            else:
                if counter > 0:
                    counter -= 1
                else:
                    counter += 1
                    ans += 1
                    
        return ans

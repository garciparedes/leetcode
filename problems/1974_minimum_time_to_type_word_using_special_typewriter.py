class Solution:
    def minTimeToType(self, word: str) -> int:
        rang = (ord('z') - ord('a') + 1)
        ans = 0
        current = "a"
        for character in word:
            ans += 1
            
            d = abs(ord(current) - ord(character))
            
            ans += min(d, rang - d)
            
            current = character 
        return ans

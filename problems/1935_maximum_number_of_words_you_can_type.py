class Solution:
    def canBeTypedWords(self, text: str, brokenLetters: str) -> int:
        broken = set(brokenLetters)
        
        ans = 0
        for word in text.split():
            if set(word) & broken:
                continue
            ans += 1
            
        return ans

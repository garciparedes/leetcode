class Solution:
    def longestNiceSubstring(self, s: str) -> str:
        
        best = str()
        
        for i in range(len(s)):
            for j in range(i + 1 + len(best), len(s) + 1):
                if self.is_nice(s[i:j]):
                    best = s[i:j]
            
            
        return best
    
    @staticmethod
    def is_nice(s: str) -> bool:
        letters = set(s)
        for l in letters:
            if l.isupper() and l.lower() not in letters:
                return False
            elif l.islower() and l.upper() not in letters:
                return False
        return True
            

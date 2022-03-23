class Solution:
    def partition(self, s: str) -> List[List[str]]:
        
        ans = self.dfs(s)
            
        return ans
    
    def dfs(self, s: str) -> List[List[str]]:      
        
        ans = list()
        
        if self.is_palindrome(s):
            ans.append([s])

        for i in range(1, len(s)):
            if not self.is_palindrome(s[:i]):
                continue
                                            
            for tail in self.dfs(s[i:]):
                ans.append([s[:i], *tail])
                
        return ans
            
                
    @staticmethod
    def is_palindrome(s: str):
        for i in range(len(s) // 2):
            if s[i] != s[len(s) - (i + 1)]:
                return False
        return True

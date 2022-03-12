class Solution:
    MAPPER = {
        "2": ["a", "b", "c"],
        "3": ["d", "e", "f"],
        "4": ["g", "h", "i"],
        "5": ["j", "k", "l"],
        "6": ["m", "n", "o"],
        "7": ["p", "q", "r", "s"],
        "8": ["t", "u", "v"],
        "9": ["w", "x", "y", "z"],
    }
    
    def letterCombinations(self, digits: str) -> List[str]:
        ans = list()
        
        self._product(digits, "", ans)
        
        return ans
    
    def _product(self, digits: str, current: str, ans: List[str]):
        if not len(digits):
            if len(current):
                ans.append(current)
            return
        
        for d in self.MAPPER[digits[0]]:
            self._product(digits[1:], current + d, ans)
        

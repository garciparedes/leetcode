from collections import Counter

class Solution:
    def countWords(self, words1: List[str], words2: List[str]) -> int:
        c1 = Counter(words1)
        c2 = Counter(words2)
        
        ans = 0
        for k1, v1 in c1.items():
            if v1 == 1 and k1 in c2 and c2[k1] == 1:
                ans += 1
        return ans

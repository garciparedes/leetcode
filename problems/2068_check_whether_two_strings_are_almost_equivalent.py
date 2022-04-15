from collections import Counter


class Solution:
    def checkAlmostEquivalent(self, word1: str, word2: str) -> bool:
        counter1 = Counter(word1)
        counter2 = Counter(word2)
        
        for c in counter1.keys() | counter2.keys():
            if abs(counter1.get(c, 0) - counter2.get(c, 0)) > 3:
                return False
        return True

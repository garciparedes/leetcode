from collections import defaultdict

class Solution:
    def areOccurrencesEqual(self, s: str) -> bool:
        counter = defaultdict(int)
        for c in s:
            counter[c] += 1
            
        iterator = iter(counter.values())
        count = next(iterator)
        for c in iterator:
            if c != count:
                return False
        return True

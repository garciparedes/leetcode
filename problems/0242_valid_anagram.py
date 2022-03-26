from collections import defaultdict


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s_counter = defaultdict(int)
        t_counter = defaultdict(int)
        
        for s_i in s:
            s_counter[s_i] += 1
            
        for t_i in t:
            t_counter[t_i] += 1
            
        return s_counter == t_counter

from collections import Counter


class Solution:
    def minSteps(self, s: str, t: str) -> int:
        s_counter, t_counter = Counter(s), Counter(t)
        
        ans = 0
        for k in s_counter.keys() | t_counter.keys():
            ans += abs(s_counter.get(k, 0) - t_counter.get(k, 0))
        return ans

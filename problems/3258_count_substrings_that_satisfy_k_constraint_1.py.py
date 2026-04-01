class Solution:
    def countKConstraintSubstrings(self, s: str, k: int) -> int:
        cum = [0]
        current = 0
        for c in s:
            if c == "1":
                current += 1
            cum.append(current)
        
        ans = 0
        for i in range(len(cum) - 1):
            for j in range(i + 1, len(cum)):
                cum_i, cum_j = cum[i], cum[j]
                cum_range = cum_j - cum_i
                cum_len = j - i
                
                if (cum_range <= k or cum_len - cum_range <= k):
                    ans += 1
        return ans

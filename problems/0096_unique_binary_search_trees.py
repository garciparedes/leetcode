class Solution:
    def numTrees(self, n: int) -> int:
        memo = [0 for _ in range(n + 1)]
        memo[0] = 1
        memo[1] = 1
        
        for i in range(2, n + 1):
            for j in range(1, i + 1):
                memo[i] += memo[j - 1] * memo[i - j]
                
        return memo[n]
                

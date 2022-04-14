class Solution:
    def twoEggDrop(self, n: int) -> int:
        dp = [0 for _ in range(n + 1)]
        
        self._optimize(dp, n)
        
        return dp[n]
        
    
    def _optimize(self, dp: list[int], n: int) -> int:
        if dp[n] == 0:
            for i in range(1, n + 1):
                dp[n] = min(
                    n if dp[n] == 0 else dp[n],
                    max(i, 1 + self._optimize(dp, n - i))
                )
        
        return dp[n]

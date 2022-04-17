class Solution:
    def countEven(self, num: int) -> int:
        ans = 0
        for i in range(1, num + 1):
            if self._digit_sum(i) % 2 == 0:
                ans += 1
        return ans
    
    def _digit_sum(self, num: int) -> int:
        ans = 0
        while num > 0:
            ans += num % 10
            num //=  10
        return ans

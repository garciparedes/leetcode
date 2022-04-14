class Solution:
    def countTriples(self, n: int) -> int:
        numbers = set(i ** 2 for i in range(1, n + 1))
        
        ans = 0
        for a in numbers:
            for b in numbers:
                if a + b in numbers:
                    ans += 1
        return ans

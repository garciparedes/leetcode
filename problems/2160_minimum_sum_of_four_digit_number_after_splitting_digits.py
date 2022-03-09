class Solution:
    def minimumSum(self, num: int) -> int:
        digits = list()
        for _ in range(4):
            digits.append(num % 10)
            num //= 10
        digits.sort()
 
        ans = digits[0] * 10 + digits[1] * 10 + digits[2] + digits[3]
        return ans

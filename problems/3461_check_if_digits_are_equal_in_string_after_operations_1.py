class Solution:
    def hasSameDigits(self, s: str) -> bool:
        digits = [int(c) for c in s]
        while len(digits) > 2:
            digits = self.transform_digits(digits)
        return digits[0] == digits[1]

    def transform_digits(self, digits: List[int]) -> List[int]:
        return [(a + b) % 10 for a, b in zip(digits[:-1], digits[1:])]
            

class Solution:
    def maxScoreSightseeingPair(self, values: List[int]) -> int:
        best = 0
        current = 0
        for value in values:
            if best < current + value:
                best = current + value
            if current < value:
                current = value
            current -= 1
        return best

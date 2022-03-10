class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        ans = 0
        previous = 0
        for row in bank:
            current = row.count("1")
            if current > 0:
                ans += previous * current
                previous = current
        return ans

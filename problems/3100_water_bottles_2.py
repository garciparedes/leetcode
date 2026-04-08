class Solution:
    def maxBottlesDrunk(self, numBottles: int, numExchange: int) -> int:
        ans = numBottles
        empty = numBottles
        exchange = numExchange
        while exchange <= empty:
            empty -= exchange
            exchange += 1
            empty += 1
            ans += 1

        return ans

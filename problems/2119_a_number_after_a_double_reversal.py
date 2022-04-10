class Solution:
    def isSameAfterReversals(self, num: int) -> bool:
        return num == self._reverse(self._reverse(num))
    
    @staticmethod
    def _reverse(num: int) -> int:
        return int(str(num)[::-1])

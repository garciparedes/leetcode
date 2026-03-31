from functools import reduce

class Solution:
    def doesValidArrayExist(self, derived: List[int]) -> bool:
        return reduce(lambda a, b: a ^ b, derived) == False

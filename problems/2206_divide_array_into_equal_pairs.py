from collections import defaultdict

class Solution:
    def divideArray(self, nums: List[int]) -> bool:
        counter = defaultdict(int)
        
        for num in nums:
            counter[num] += 1
            
        for count in counter.values():
            if count % 2 == 1:
                return False
            
        return True

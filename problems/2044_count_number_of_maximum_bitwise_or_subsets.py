from collections import defaultdict

class Solution:
    def countMaxOrSubsets(self, nums: List[int]) -> int:
        counter = defaultdict(int)
        counter[0] += 1
        
        for num in nums:
            for k, v in tuple(counter.items()):
                counter[k | num] += v
                
        ans = counter[max(counter.keys())]
        return ans

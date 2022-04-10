from collections import defaultdict


class Solution:
    def numOfPairs(self, nums: List[str], target: str) -> int:
        ans = 0
        seen = defaultdict(int)
        for num in nums:
            if target.startswith(num) and target[len(num):] in seen:
                ans += seen[target[len(num):]]
                
            if target.endswith(num) and target[:len(target) - len(num)] in seen:
                ans += seen[target[:len(target) - len(num)]]
            
            seen[num] += 1
            
        return ans

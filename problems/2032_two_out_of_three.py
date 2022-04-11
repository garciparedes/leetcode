from collections import defaultdict


class Solution:
    def twoOutOfThree(self, nums1: List[int], nums2: List[int], nums3: List[int]) -> List[int]:
        counter = defaultdict(int)
        
        for num in set(nums1):
            counter[num] += 1
        for num in set(nums2):
            counter[num] += 1
        for num in set(nums3):
            counter[num] += 1
        
        ans = list()
        for num, v in counter.items():
            if v > 1:
                ans.append(num)
            
        return ans

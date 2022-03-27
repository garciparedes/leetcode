from collections import defaultdict

class Solution:
    def fourSumCount(self, nums1: List[int], nums2: List[int], nums3: List[int], nums4: List[int]) -> int:
        counter = defaultdict(int)
        for n1 in nums1:
            for n2 in nums2:
                counter[n1 + n2] += 1
                
        ans = 0
        
        for n3 in nums3:
            for n4 in nums4:
                ans += counter.get(-(n3 + n4), 0)
                    
        return ans

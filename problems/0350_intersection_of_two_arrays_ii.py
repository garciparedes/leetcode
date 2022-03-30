from collections import defaultdict

class Solution:
    def intersect(self, nums1: List[int], nums2: List[int]) -> List[int]:
        c1 = defaultdict(int)
        c2 = defaultdict(int)
        
        for n1 in nums1:
            c1[n1] += 1
            
        for n2 in nums2:
            c2[n2] += 1
            
        ans = list()
        for k in c1.keys() & c2.keys():
            ans += [k] * min(c1[k], c2[k])
            
        return ans

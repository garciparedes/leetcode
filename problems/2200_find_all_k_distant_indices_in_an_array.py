from math import ceil 


class Solution:
    def findKDistantIndices(self, nums: List[int], key: int, k: int) -> List[int]:
        key_indexes = list()
        for i in range(len(nums)):
            if nums[i] == key:
                key_indexes.append(i)
                
        ans = list()
        for i in range(len(nums)):
            if self._is_k_distant(key_indexes, i, k):
                ans.append(i)
                
        return ans
    
    @staticmethod
    def _is_k_distant(indexes: list[int], i: int, k: int) -> bool:
        
        left, right = 0, len(indexes) - 1
        while left < right:
            mid = ceil((left + right) /  2)
            
            if indexes[mid] > i:
                right = mid - 1
            else:
                left = mid
                
        if abs(indexes[left] - i) <= k:
            return True
        
        if left < len(indexes) - 1:
            left += 1
            return abs(indexes[left] - i) <= k
        
        return False
        
        
        
        

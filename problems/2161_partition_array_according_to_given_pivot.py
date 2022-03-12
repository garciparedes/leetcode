class Solution:
    def pivotArray(self, nums: List[int], pivot: int) -> List[int]:
        before = list()
        after = list()
        
        pivot_count = 0
        for num in nums:
            if num < pivot:
                before.append(num)
            elif num > pivot:
                after.append(num)
            else:   
                pivot_count += 1
                
        ans = before + ([pivot] * pivot_count) + after
        
        return ans

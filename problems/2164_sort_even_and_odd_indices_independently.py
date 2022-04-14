class Solution:
    def sortEvenOdd(self, nums: List[int]) -> List[int]:
        odds = list()
        evens = list()
        
        for i, num in enumerate(nums):
            if i % 2:
                evens.append(num)
            else:   
                odds.append(num)
                        
        odds.sort()
        evens.sort(reverse=True)
        
        for i in range(len(nums)):
            if i % 2:
                nums[i] = evens[i // 2]
            else:
                nums[i] = odds[i // 2]
                
        return nums

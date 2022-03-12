class Solution:
    def minPairSum(self, nums: List[int]) -> int:
        nums.sort()
        
        for i in range(len(nums) // 2):
            nums[i] += nums[len(nums) - i - 1]
                
        ans = max(nums[:len(nums) //2])
        
        return ans

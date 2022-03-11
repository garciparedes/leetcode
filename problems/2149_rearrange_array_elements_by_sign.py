class Solution:
    def rearrangeArray(self, nums: List[int]) -> List[int]:
        positives = list()
        negatives = list()
        for num in nums:
            if num > 0:
                positives.append(num)
            else:
                negatives.append(num)
                        
        ans = list()
        for i in range(len(nums) // 2):
            ans.append(positives[i])
            ans.append(negatives[i])
    
        return ans

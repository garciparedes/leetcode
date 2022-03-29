from random import randrange

class Solution:

    def __init__(self, nums: List[int]):
        self.base = nums
        
    def reset(self) -> List[int]:
        return self.base

    def shuffle(self) -> List[int]:
        ans = self.base.copy()
        for i in range(len(ans)):
            j = randrange(len(ans))
            ans[i], ans[j] = ans[j], ans[i]
        return ans
                
            


# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.reset()
# param_2 = obj.shuffle()

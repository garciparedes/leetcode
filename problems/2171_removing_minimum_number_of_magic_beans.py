class Solution:
    def minimumRemoval(self, beans: List[int]) -> int:
        beans.sort()
        total = sum(beans)
        
        best = total
        cum = 0
        for i, bean in enumerate(beans):
            curr = total - bean * (len(beans) - i)
            cum += bean
            
            if curr < best:
                best = curr
            
        return best

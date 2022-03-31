class Solution:
    def trap(self, height: List[int]) -> int:
        left, right = 0, len(height) -1
        
        left_higher = 0
        right_higher = 0
        
        ans = 0
        while left < right:
            
            if height[left] <= height[right]:
                if height[left] > left_higher:
                    left_higher = height[left]
                else:
                    ans += left_higher - height[left]
                left += 1
            else:
                if height[right] > right_higher:
                    right_higher = height[right]
                else:
                    ans += right_higher - height[right]
                right -= 1
            
            
        return ans
        
    
                
            
                

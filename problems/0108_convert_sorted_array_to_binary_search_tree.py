# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        return self._array_to_tree(nums)
        
        
    def _array_to_tree(self, nums: List[int]) -> Optional[TreeNode]:
        if not len(nums):
            return None
        
        mid = len(nums) // 2
        
        left = self._array_to_tree(nums[:mid])
        right = self._array_to_tree(nums[mid + 1:])
        
        return TreeNode(nums[mid], left, right)
        

from collections import deque

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return list()
        
        queue = collections.deque()
        queue.append((root, 0))
        
        ans = list()
        
        while len(queue):
            node, depth = queue.popleft()
            
            while len(ans) <= depth:
                ans.append(list())
                
            ans[depth].append(node.val)
            
            if node.left is not None:
                queue.append((node.left, depth + 1))
                
            if node.right is not None:
                queue.append((node.right, depth + 1))
                
        return ans
            
        
        

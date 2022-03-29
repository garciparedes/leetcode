# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        p_path = self._search(root, p)
        q_path = self._search(root, q)
        
        ans = root
        for pi, qi in zip(p_path, q_path):
            if pi != qi:
                break
            ans = pi
            
        return ans
        
    def _search(self, root, target) -> Optional[List[TreeNode]]:
        if root is None:
            return None
        
        if root.val == target.val:
            return [root]
        
        left = self._search(root.left, target)
        if left is not None:
            return [root] + left
        
        right = self._search(root.right, target)
        if right is not None:
            return [root] + right
        
        return None

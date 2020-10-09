from collections import deque

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

    def serialize(self, root: TreeNode) -> str:
        if root is None:
            return ""
        ans = str()
        s = list()
        s.append(root)
        while len(s):
            item = s.pop()
            ans += f"{item.val},"
            if item.left:
                s.append(item.left)
            if item.right:
                s.append(item.right)
        
        return ans[:-1]

    def deserialize(self, data: str) -> TreeNode:
        """Decodes your encoded data to tree.
        """
        if not len(data):
            return None
    
        queue = deque(map(int, data.split(",")))
        if not len(queue):
            return None
        queue.reverse()
        
        def get_node(q) -> TreeNode:
            if not len(q):
                return None
            root = TreeNode(q.pop())
            tmp_q = deque()
            while len(q) and q[-1] > root.val:
                tmp_q.appendleft(q.pop())
            root.left = get_node(q);
            root.right = get_node(tmp_q);
            return root
        
        return get_node(queue)

# Your Codec object will be instantiated and called as such:
# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# tree = ser.serialize(root)
# ans = deser.deserialize(tree)
# return ans

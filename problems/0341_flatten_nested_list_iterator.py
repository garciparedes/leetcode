# """
# This is the interface that allows for creating nested lists.
# You should not implement it, or speculate about its implementation
# """
#class NestedInteger:
#    def isInteger(self) -> bool:
#        """
#        @return True if this NestedInteger holds a single integer, rather than a nested list.
#        """
#
#    def getInteger(self) -> int:
#        """
#        @return the single integer that this NestedInteger holds, if it holds a single integer
#        Return None if this NestedInteger holds a nested list
#        """
#
#    def getList(self) -> [NestedInteger]:
#        """
#        @return the nested list that this NestedInteger holds, if it holds a nested list
#        Return None if this NestedInteger holds a single integer
#        """

class NestedIterator:
    def __init__(self, nestedList: [NestedInteger]):
        self.values = nestedList
        self.index = [-1]
        self._next_index()

    def next(self) -> int:   
        curr = self._get_value()
        self._next_index()
        ans = curr.getInteger()
        return ans
    
    def _next_index(self):        
        while len(self.index):
            self.index[-1] += 1
            curr = self._get_value()
            
            if curr is None:
                self.index.pop()
            elif isinstance(curr, list):
                self.index.append(-1)
            else:
                break
        
                
    def _get_value(self) -> Optional[NestedInteger]:
        curr = self.values
        for i in self.index:
            if len(curr) <= i:
                return None
            curr = curr[i]
            if not curr.isInteger():
                curr = curr.getList() 
        return curr
                          
    
    def hasNext(self) -> bool:
        return len(self.index)
         

# Your NestedIterator object will be instantiated and called as such:
# i, v = NestedIterator(nestedList), []
# while i.hasNext(): v.append(i.next())

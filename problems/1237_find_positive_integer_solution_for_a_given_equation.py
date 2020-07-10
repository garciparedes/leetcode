"""
   This is the custom function interface.
   You should not implement it, or speculate about its implementation
   class CustomFunction:
       # Returns f(x, y) for any given positive integers x and y.
       # Note that f(x, y) is increasing with respect to both x and y.
       # i.e. f(x, y) < f(x + 1, y), f(x, y) < f(x, y + 1)
       def f(self, x, y):
  
"""

class Solution:
    def findSolution(self, customfunction: 'CustomFunction', z: int) -> List[List[int]]:
        fn = customfunction.f
        x, y = 1, 1000
        
        valid = list()
        while y >= 1 and x <= 1000:
            val = fn(x, y)
            if val == z:
                valid.append([x, y])
                y -= 1
            elif val < z:
                x += 1
            else:
                y -= 1
        
        return valid

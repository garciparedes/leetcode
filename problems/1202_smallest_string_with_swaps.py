from collections import defaultdict

class UnionFind:
    def __init__(self, n: int):
        self.p = list(range(n))
        
    def union(self, x: int, y: int) -> None:
        self.p[self.find(x)] = self.find(y)
        
    def find(self, x: int) -> int:
        if x != self.p[x]:
            self.p[x] = self.find(self.p[x])
        return self.p[x]
    

class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        union_find = UnionFind(len(s))
        
        for x, y in pairs:
            union_find.union(x, y)
        
        m = defaultdict(list)            
        for i in range(len(s)):
            m[union_find.find(i)].append(s[i])

        for comp_id in m.keys():
            m[comp_id].sort(reverse=True)
        
        ans = list()
        for i in range(len(s)):
            ans.append(m[union_find.find(i)].pop())
            
        return "".join(ans)
        

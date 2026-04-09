from collections import defaultdict
from itertools import combinations

class Solution:
    def countCompleteComponents(self, n: int, edges: List[List[int]]) -> int:
        adjacency = defaultdict(set)
        for edge in edges:
            adjacency[edge[0]].add(edge[1])
            adjacency[edge[1]].add(edge[0])

        components = list()
        visited = [False for _ in range(n)]
        for current in range(n):
            if not visited[current]:
                component = set()
                self.dfs(adjacency, current, visited, component)
                components.append(component)

        ans = 0
        for component in components:
            if self.is_complete(adjacency, component):
                ans += 1
        return ans 

    def dfs(self, adjacency: Map[int, Set[int]], current: int, visited: List[bool], component: Set[int]): 
        if visited[current]:
            return 
        visited[current] = True
        component.add(current)
        for another in adjacency[current]:
            self.dfs(adjacency, another, visited, component)

    def is_complete(self, adjacency: Map[int, Set[int]], component: Set[int]) -> bool:
        for (current, another) in combinations(component, 2):
            if current != another and another not in adjacency[current]:
                return False
        return True

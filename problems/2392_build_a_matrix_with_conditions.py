from collections import defaultdict

class Solution:
    def buildMatrix(self, k: int, rowConditions: List[List[int]], colConditions: List[List[int]]) -> List[List[int]]:
        rows = self.generate_sorted(k, rowConditions)
        cols = self.generate_sorted(k, colConditions)

        if (len(rows) == 0 or len(cols) == 0):
            return []

        ans = [[0 for _ in range(k)] for _ in range(k)]
        for i in range(k):
            ans[rows.index(i)][cols.index(i)] = i + 1

        return ans

    def generate_sorted(self, k: int, conditions: List[List[int]]) -> List[int]:        
        adjacency = defaultdict(set)
        for condition in conditions:
            adjacency[condition[0] - 1].add(condition[1] - 1)

        sequence = list()
        visited = [0 for _ in range(k)]
        for current in range(k):
            if not self.dfs(adjacency, visited, current, sequence):
                return []
        sequence.reverse()

        return sequence

    def dfs(self, adjacency: Dict[int, Set[int]], visited: List[int], current: int, sequence: List[int]) -> bool:
        if visited[current] == 2:
            return True
        if visited[current] == 1:
            return False

        visited[current] = 1
        for another in adjacency[current]:
            if not self.dfs(adjacency, visited, another, sequence):
                return False
        visited[current] = 2

        sequence.append(current)
        return True
        

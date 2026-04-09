from collections import Counter

class Solution:
    def mergeSimilarItems(self, items1: List[List[int]], items2: List[List[int]]) -> List[List[int]]:
        first = Counter()
        for key, value in items1:
            first[key] += value

        second = Counter()
        for key, value in items2:
            second[key] += value

        ans = list()
        for key in sorted(first.keys() | second.keys()):
            ans.append([key, first[key] + second[key]])
        return ans

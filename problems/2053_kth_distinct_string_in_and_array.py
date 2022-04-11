class Solution:
    def kthDistinct(self, arr: List[str], k: int) -> str:
        uniques = dict()
        for s in arr:
            uniques[s] = (s not in uniques)

        count = 0
        for s, v in uniques.items():
            if not v:
                continue
            
            count += 1
            if count == k:
                return s
                
        return str()

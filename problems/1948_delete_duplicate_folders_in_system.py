from collections import Counter

class Solution:
    def deleteDuplicateFolder(self, paths: List[List[str]]) -> List[List[str]]:
        root = self.create_tree(paths)
        counter = Counter()
        self.compute_hash(root, counter)
        filtered = self.filter_hashes(counter)
        pruned = self.prune_tree(root, filtered)
        
        ans = list()
        self.populate(pruned, ans, [])
        return ans

    def create_tree(self, paths: List[List[str]]) -> Dict[str, Dict]:
        root = dict()
        for path in paths:
            current = root
            for level in path:
                current[level] = current.get(level, dict())
                current = current[level]
        return root

    def compute_hash(self, root: Dict[str, Dict], counter: Optional[Counter[str]] = None) -> Dict[str, Dict]:
        hashes = dict()
        for key, value in root.items():
            hashed = self.compute_hash(value, counter)
            v = "".join(sorted(hashed.keys()))
            if len(value) > 0 and counter is not None:
                counter[f"(*{v})"] += 1
            hashes[f"({key}{v})"] = hashed 
        return hashes
    
    def filter_hashes(self, counter: Counter[str]) -> Set[str]:
        ans = set()
        for key, value in counter.items():
            if value > 1:
                ans.add(key)
        return ans

    def prune_tree(self, root: Dict[str, Dict], hashes: Set[str]) -> Dict[str, Dict]:
        ans = dict()
        for key, value in root.items():
            pruned = self.prune_tree(value, hashes)
            hashed = self.compute_hash(value)
            v = "".join(sorted(hashed.keys()))
            if f"(*{v})" not in hashes:
                ans[key] = pruned
        return ans

    def populate(self, root: Dict[str, Dict], ans: List[List[str]], current: List[str]):
        if len(current) > 0:
            ans.append(current.copy())
        for key, value in root.items():
            current.append(key)
            self.populate(value, ans, current)
            current.pop()

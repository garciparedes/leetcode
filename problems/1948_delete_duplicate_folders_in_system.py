from collections import Counter

class Solution:
    def deleteDuplicateFolder(self, paths: List[List[str]]) -> List[List[str]]:
        root = Node.from_paths(paths)
        root.prune_duplicates()
        return root.to_paths()
        
class Node:

    def __init__(self, folder: Optional[str] = None) -> None:
        self._folder = folder
        self._children = dict()
        self._hash = None
        self._generic_hash = None

    @staticmethod
    def from_paths(paths: List[List[str]]) -> Node:
        root = Node()
        for path in paths:
            root._add_path(path)
        root._compute_hash()
        return root

    def prune_duplicates(self) -> None:
        hashes = self._find_duplicates()
        self._prune(hashes)

    def to_paths(self) -> List[List[str]]: 
        ans = list()
        current = list()
        self._to_paths(ans, current)
        return ans
   
    @staticmethod
    def _filter_hashes(counter: Counter[str]) -> Set[str]:
        ans = set()
        for key, value in counter.items():
            if value > 1:
                ans.add(key)
        return ans

    def _to_paths(self, ans: List[List[str]], current: List[str]): 
        if len(current) > 0:
            ans.append(current.copy())
        for key, value in self._children.items():
            current.append(key)
            value._to_paths(ans, current)
            current.pop()

    def _add_path(self, path: List[str]) -> None:
        current = self
        for level in path:
            current = current._get_child(level)

    def _get_child(self, folder: str) -> None:
        child = self._children.get(folder, None)
        if child is None:
            child = Node(folder)
            self._children[folder] = child
        return child

    def _find_duplicates(self) -> Set[str]:
        counter = Counter()
        self._count_patterns(counter)
        return self._filter_hashes(counter)

    def _count_patterns(self, counter: Counter[str]) -> None:
        for key, value in self._children.items():
            value._count_patterns(counter)
            if len(value._children) > 0:
                counter[value._generic_hash] += 1
    
    def _prune(self, hashes: Set[str]) -> None:
        for key in list(self._children.keys()):
            value = self._children[key]
            if value._generic_hash in hashes:
                self._children.pop(key)
            else:
                value._prune(hashes)

    def _compute_hash(self) -> None:
        for key, value in self._children.items():
            value._compute_hash()
        self._hash, self._generic_hash = self._compose_hash()

    def _compose_hash(self) -> Tuple[str, str]:
        suffix =  "".join(sorted(map(lambda node: node._hash, self._children.values())))
        return f"{self._folder}{suffix})", f"(*{suffix})"


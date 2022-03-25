class Trie:

    def __init__(self):
        self.tree = dict()

        
    def insert(self, word: str) -> None:
        current = self.tree
        for w in word:
            if w not in current:
                current[w] = dict()
            current = current[w]
        
        if "." not in current:
            current["."] = None

    def search(self, word: str) -> bool:
        current = self.tree
        for w in word:
            if w not in current:
                return False
            current = current[w]
        return "." in current

    def startsWith(self, prefix: str) -> bool:
        current = self.tree
        for w in prefix:
            if w not in current:
                return False
            current = current[w]
        return True


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)

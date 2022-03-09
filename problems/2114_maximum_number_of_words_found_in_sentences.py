class Solution:
    def mostWordsFound(self, sentences: List[str]) -> int:
        best = 0
        for sentence in sentences:
            current = 1
            for c in sentence:
                if c == " ":
                    current += 1
            if best < current:
                best = current
        return best

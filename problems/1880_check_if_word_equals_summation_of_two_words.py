class Solution:
    def isSumEqual(self, firstWord: str, secondWord: str, targetWord: str) -> bool:
        return self._to_number(firstWord) + self._to_number(secondWord) == self._to_number(targetWord)

    @staticmethod
    def _to_number(word: str) -> int:
        parts = list()
        for c in word:
            part = str(ord(c) - ord("a"))
            parts.append(part)
        
        full = "".join(parts)
        
        return int(full)    

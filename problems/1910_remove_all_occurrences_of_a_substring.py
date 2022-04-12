class Solution:
    def removeOccurrences(self, s: str, part: str) -> str:
        while (index := s.find(part)) != -1:
            s = s[:index] + (s[index + len(part):] or "")
        return s

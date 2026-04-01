class Solution:
    def makeLargestSpecial(self, s: str) -> str:
        return self.optimize(s)

    def optimize(self, s: str) -> str:
        parts = list()
        current = ""
        height = 0
        for c in s:
            if c == "1":
                height += 1
            else:
                height -= 1
            current += c
            if height == 0:
                parts.append(current)
                current = ""
        
        for i, part in enumerate(parts):
            parts[i] = part[0] + self.optimize(part[1:-1]) + part[-1]
        parts.sort(reverse=True)
        
        return "".join(parts)

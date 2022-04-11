class Solution:
    def checkString(self, s: str) -> bool:
        a_mode = True
        for c in s:
            if c == "a" and not a_mode:
                return False
            elif c == "b" and a_mode:
                a_mode = False
        return True
            

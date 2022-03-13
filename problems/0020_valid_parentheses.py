class Solution:
    def isValid(self, s: str) -> bool:
        stack = list()
        
        for bracket in s:
            if bracket in ("(", "[", "{"):
                stack.append(bracket)
            elif (
                len(stack) 
                and (
                    (bracket == ")" and stack[-1] == "(")
                    or (bracket == "]" and stack[-1] == "[")
                    or (bracket == "}" and stack[-1] == "{")
                )
            ):
                stack.pop(-1)
            else:
                return False
            
        if len(stack):
            return False
        
        return True
            

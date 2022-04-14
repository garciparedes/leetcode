class Solution:
    def areNumbersAscending(self, s: str) -> bool:
        current = -1
        for word in s.split():
            if word.isnumeric():
                tmp = int(word)
                if current >= tmp:
                    return False
                current = tmp
        return True

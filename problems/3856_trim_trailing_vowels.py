class Solution:
    def trimTrailingVowels(self, s: str) -> str:
        characters = list(s)
        while len(characters) > 0 and characters[-1] in "aeiou":
            characters.pop()
        return "".join(characters)

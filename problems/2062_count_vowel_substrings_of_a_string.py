VOWELS = {"a", "e", "i", "o", "u"}


class Solution:
    def countVowelSubstrings(self, word: str) -> int:
        ans = 0
        last_seen_consonant = -1
        last_seen_vowel = {v: -1 for v in VOWELS}
        for i in range(len(word)):
            c = word[i]
            if c in VOWELS:
                last_seen_vowel[c] = i
                ans += max(min(last_seen_vowel.values()) - last_seen_consonant, 0)
            else:
                last_seen_consonant = i
        return ans

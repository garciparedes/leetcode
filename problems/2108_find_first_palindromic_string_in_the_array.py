class Solution:
    def firstPalindrome(self, words: List[str]) -> str:
        ans = str()
        for word in words:
            if self.is_palindrome(word):
                ans = word
                break
        return ans

    def is_palindrome(self, word: str):
        for i in range(len(word) // 2):
            if word[i] != word[len(word) - i - 1]:
                return False
        return True

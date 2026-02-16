class Solution:
    def mapWordWeights(self, words: List[str], weights: List[int]) -> str:
        return "".join(map(lambda word: self.mapWord(word, weights), words))

    def mapWord(self, word: str, weights: List[int]) -> str: 
        weight = sum(map(lambda c: weights[ord(c) - ord('a')], word))
        return chr(ord('z') - (weight % 26))

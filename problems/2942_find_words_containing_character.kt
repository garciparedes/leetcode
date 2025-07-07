class Solution {
    fun findWordsContaining(words: Array<String>, x: Char): List<Int> {
        val ans = mutableListOf<Int>()
        words.forEachIndexed { index, word ->
            if (word.contains(x)) {
                ans.add(index)
            }
        }
        return ans
    }
}

class Solution {
    fun maximumNumberOfStringPairs(words: Array<String>): Int {
        val reverses = mutableSetOf<String>()
        var ans = 0
        words.forEach { word ->
            if (reverses.contains(word)) {
                ans += 1
            }
            reverses.add(word.reversed())
        }
        return ans
    }
}

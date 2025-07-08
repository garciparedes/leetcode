class Solution {
    fun maxFreqSum(s: String): Int {
        val vowelCounter = mutableMapOf<Char, Int>()
        val consonantCounter = mutableMapOf<Char, Int>()

        s.forEach { c ->
            if (vowels.contains(c)) {
                vowelCounter.merge(c, 1, Int::plus)
            } else {
                consonantCounter.merge(c, 1, Int::plus)
            }
        }

        val maxVowel = vowelCounter.values.maxOrNull() ?: 0
        val maxConsonant = consonantCounter.values.maxOrNull() ?: 0

        return maxVowel + maxConsonant
    }

    companion object {
        val vowels = setOf('a', 'e', 'i', 'o', 'u')
    }
}

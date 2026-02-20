class Solution {
    fun stringSequence(target: String): List<String> {
        val ans = mutableListOf<String>()
        var current = ""
        for (value in target) {
            for (c in 'a'..value) {
                ans.add(current + c)
            }
            current += value
        }
        return ans
    }
}

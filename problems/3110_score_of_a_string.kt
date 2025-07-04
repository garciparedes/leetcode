class Solution {
    fun scoreOfString(s: String): Int {
        val iterator = s.toCharArray().iterator()
        if (!iterator.hasNext()) {
            return 0
        }
        var ans = 0
        var previous = iterator.next().code
        while (iterator.hasNext()) {
            val current = iterator.next().code
            ans += abs(current - previous)
            previous = current
        }
        return ans
    }
}

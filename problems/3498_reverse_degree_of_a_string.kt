class Solution {
    fun reverseDegree(s: String): Int {
        var ans = 0
        s.toCharArray().forEachIndexed { i, c ->
            ans += (i + 1) * (123 - c.code)
        }
        return ans
    }
}

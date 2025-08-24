class Solution {
    fun countKeyChanges(s: String): Int {
        var ans = 0
        s.windowed(2).forEach { w ->
            if (w[0].lowercaseChar() != w[1].lowercaseChar()) {
                ans += 1
            }
        }
        return ans
    }
}

class Solution {
    fun finalString(s: String): String {
        var ans = ""
        for (c in s) {
            if (c == 'i') {
                ans = ans.reversed()
            } else {
                ans += c
            }
        }
        return ans
    }
}

class Solution {
    fun pivotInteger(n: Int): Int {
        var left = 0
        var right = n * (n + 1) / 2
        var ans = 1
        while (left < right) {
            left += ans
            if (left == right) {
                return ans
            }
            right -= ans
            ans += 1
        }
        return -1
    }
}

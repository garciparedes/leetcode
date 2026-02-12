class Solution {
    fun mirrorDistance(n: Int): Int = abs(n - reverse(n))

    private fun reverse(n: Int): Int {
        var curr = n
        var ans = 0
        while (curr > 0) {
            ans *= 10
            ans += curr % 10
            curr /= 10
        }
        return ans
    }
}

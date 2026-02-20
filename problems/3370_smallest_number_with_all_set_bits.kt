class Solution {
    fun smallestNumber(n: Int): Int {
        var ans = 1
        while (ans < n) {
            ans =  (ans shl 1) + 1
        }
        return ans
    }
}

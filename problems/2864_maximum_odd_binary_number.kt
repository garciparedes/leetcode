class Solution {
    fun maximumOddBinaryNumber(s: String): String {
        val onesCount = s.fold(0) { acc, c -> acc + if (c == '1') { 1 } else { 0 }}
        val ans = StringBuilder(s.length)
        repeat(onesCount - 1) {
            ans.append('1')
        }
        repeat(s.length - onesCount) {
            ans.append('0')
        }
        ans.append('1')
        return ans.toString()
    }
}

class Solution {
    fun stringHash(s: String, k: Int): String {
        var counter = 0
        var hashSum = 0
        val ans = StringBuilder(s.length / k)
        s.forEach { c ->
            counter += 1
            hashSum += c.toInt() - 97
            if (counter == k) {
                ans.append((97 + (hashSum % 26)).toChar())
                hashSum = 0
                counter = 0
            }
        }
        return ans.toString()
    }
}

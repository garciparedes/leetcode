class Solution {
    fun countSeniors(details: Array<String>): Int {
        var ans = 0
        details.forEach { citizen ->
            if (citizen.substring(11, 13).toInt() > 60) {
                ans += 1
            }
        }
        return ans
    }
}

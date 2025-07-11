class Solution {
    fun numberOfEmployeesWhoMetTarget(hours: IntArray, target: Int): Int {
        var ans = 0
        hours.forEach {
            if(it >= target) {
                ans += 1
            }
        }
        return ans
    }
}

class Solution {
    fun smallestIndex(nums: IntArray): Int {
         nums.forEachIndexed { index, num ->
            if (index == sumDigits(num)) {
                return index
            }
         }
         return -1
    }

    fun sumDigits(num: Int): Int {
        var tmp = num
        var ans = 0
        while (tmp > 0) {
            ans += tmp % 10
            tmp /= 10
        }
        return ans
    }
}

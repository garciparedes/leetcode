class Solution {
    fun numberOfPairs(nums1: IntArray, nums2: IntArray, k: Int): Int {
        var ans = 0
        nums1.forEach { num1 ->
            nums2.forEach { num2 ->
                if (num1 % (num2 * k) == 0) {
                    ans += 1
                }
            }
        }
        return ans
    }
}

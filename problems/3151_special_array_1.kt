class Solution {
    fun isArraySpecial(nums: IntArray): Boolean {
        val parity = nums[0] % 2 == 0
        nums.forEachIndexed { i, num ->
            if ((num % 2 == 0) != ((i % 2 != 0) xor parity)) {
                return false
            }
        }
        return true
    }
}

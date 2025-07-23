class Solution {
    fun countDistinctIntegers(nums: IntArray): Int {
        val uniques = mutableSetOf<Int>()
        nums.forEach { num ->
            var reversed = 0
            var current = num
            while (current > 0) {
                reversed = reversed * 10 + current % 10
                current /= 10
            }
            uniques.add(reversed)
            uniques.add(num)
        }
        return uniques.size
    }
}

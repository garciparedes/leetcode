class Solution {
    fun getSneakyNumbers(nums: IntArray): IntArray {
        val seen = mutableSetOf<Int>()
        val duplicates = mutableListOf<Int>()
        nums.forEach { num ->
            if (seen.contains(num)) {
                duplicates.add(num)
            } else {
                seen.add(num)
            }
        }
        return duplicates.toIntArray()
    }
}

class Solution {
    fun minBitwiseArray(nums: List<Int>): IntArray {
        val mapper = createMapper()
        return nums.map { mapper.getOrDefault(it, -1) }.toIntArray()
    }

    fun createMapper(): Map<Int, Int> {
        val mapper = mutableMapOf<Int, Int>()
        for (value in 0..1000) {
            val key = value or (value + 1)
            mapper.getOrPut(key) { value }
        }
        return mapper
    }
}

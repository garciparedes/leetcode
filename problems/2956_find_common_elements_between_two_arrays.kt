class Solution {
    fun findIntersectionValues(nums1: IntArray, nums2: IntArray): IntArray {
        val set1 = nums1.toSet()
        val set2 = nums2.toSet()
        var ans1 = 0
        var ans2 = 0
        nums1.forEach { num ->
            if (set2.contains(num)) {
                ans1 += 1
            }
        }
        nums2.forEach { num ->
            if (set1.contains(num)) {
                ans2 += 1
            }
        }
        return listOf(ans1, ans2).toIntArray()
    }
}

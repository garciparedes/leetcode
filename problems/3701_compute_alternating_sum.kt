class Solution {
    fun alternatingSum(nums: IntArray): Int = nums.reduceIndexed { 
        index, acc, num -> if (index % 2 == 0) { acc + num } else { acc - num }   
    }
}

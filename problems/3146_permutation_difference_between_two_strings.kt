class Solution {
    fun findPermutationDifference(s: String, t: String): Int {
        val tMapper = t.toCharArray().mapIndexed{ index, c -> c to index }.toMap()
        var ans = 0
        s.forEachIndexed { i, c ->
            ans += abs(i - tMapper.get(c)!!)
        }
        return ans
    }
}

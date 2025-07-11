class Solution {
    fun findThePrefixCommonArray(A: IntArray, B: IntArray): IntArray {
        val setA = mutableSetOf<Int>()
        val setB = mutableSetOf<Int>()
        val ans = mutableListOf<Int>()
        A.forEachIndexed { i, a ->
            setA.add(a)
            setB.add(B[i]!!)
            ans.add(setA.intersect(setB).size)
        }
        return ans.toIntArray()
    }
}

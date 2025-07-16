class Solution {
    fun findClosest(x: Int, y: Int, z: Int): Int {
        val d1 = abs(x - z)
        val d2 = abs(y - z)
        return if (d1 < d2) {
            1
        } else if (d1 > d2) {
            2
        } else {
            0
        }
    }
}

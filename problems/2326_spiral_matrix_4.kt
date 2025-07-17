/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun spiralMatrix(m: Int, n: Int, head: ListNode?): Array<IntArray> {
        val ans = Array(m) { IntArray(n) { -1 } }
        var nn = n
        var mm = m - 1
        var counter = nn
        val currentIndex = mutableListOf(0, -1)
        var direction = 0
        var current = head
        while (current != null) {
            when (direction) {
                0 -> {
                    counter -= 1
                    currentIndex[1] += 1
                    if (counter == 0) {
                        nn -= 1
                        counter = mm
                        direction = 1
                    }
                }
                1 -> {
                    counter -= 1
                    currentIndex[0] += 1
                    if (counter == 0) {
                        mm -= 1
                        counter = nn
                        direction = 2
                    }
                }
                2 -> {
                    counter -= 1
                    currentIndex[1] -= 1
                    if (counter == 0) {
                        nn -= 1
                        counter = mm
                        direction = 3
                    }
                }
                else -> {
                    counter -= 1
                    currentIndex[0] -= 1
                    if (counter == 0) {
                        mm -= 1
                        counter = nn
                        direction = 0
                    }
                }
            }
            ans[currentIndex[0]][currentIndex[1]] = current.`val`
            current = current.next
        }
        return ans
    }
}

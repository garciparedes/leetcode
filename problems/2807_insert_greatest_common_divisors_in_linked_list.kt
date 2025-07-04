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
    fun insertGreatestCommonDivisors(head: ListNode?): ListNode? = head?.let { currentNode ->
        insertGreatestCommonDivisors(currentNode.next)?.let { nextNode ->
            val newValue = gcd(currentNode.`val`, nextNode.`val`)
            val newNode = ListNode(newValue)
            newNode.next = nextNode
            currentNode.next = newNode
        }
        return currentNode
    }

    fun gcd(a: Int, b: Int): Int {
        var aa = a
        var bb = b
        while (bb != 0) {
            val tmp = bb
            bb = aa % bb
            aa = tmp
        }
        return aa
    }
}

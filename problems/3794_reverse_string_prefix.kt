class Solution {
    fun reversePrefix(s: String, k: Int): String = s.take(k).reversed() + s.takeLast(s.length - k)
}

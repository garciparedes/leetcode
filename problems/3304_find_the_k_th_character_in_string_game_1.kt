class Solution {
    fun kthCharacter(k: Int): Char {
        if (k == 1) {
            return  'a'
        }
        var current = mutableListOf('a')
        while(true) {
            (0..<current.size).forEach { j ->
                current.add(((current[j].toInt() - 97 + 1) % 26 + 97).toChar())
                if (current.size == k) {
                    return current.last()
                }
            }
        }
        return 'E'
    }
}

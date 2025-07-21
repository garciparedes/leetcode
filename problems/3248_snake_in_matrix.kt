class Solution {
    fun finalPositionOfSnake(n: Int, commands: List<String>): Int {
        val position = mutableListOf(0, 0)
        commands.forEach { command ->
            when (command) {
                "UP" -> position[0] -= 1
                "DOWN" -> position[0] += 1
                "RIGHT" -> position[1] += 1
                else -> position[1] -= 1
            }
        }
        println(position)
        return position[0] * n + position[1]
    }
}

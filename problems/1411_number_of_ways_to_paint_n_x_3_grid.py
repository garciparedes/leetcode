class Solution:
    MODULO = 10 ** 9 + 7

    def numOfWays(self, n: int) -> int:
        two_color, three_color = 6, 6

        for _ in range(n - 1):
            old_two_color, old_three_color = two_color, three_color

            two_color = (3 * old_two_color + 2 * old_three_color) % self.MODULO
            three_color = (2 * old_two_color + 2 * old_three_color) % self.MODULO

        return (two_color + three_color) % self.MODULO

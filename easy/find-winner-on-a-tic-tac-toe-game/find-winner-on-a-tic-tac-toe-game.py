class Solution:
    def tictactoe(self, moves):
        g = [0, 0, 0, 0, 0, 0, 0, 0]

        for i, (x, y) in enumerate(moves):
            action = (i & 1) * 2 - 1
            g[x] += action
            g[3+y] += action
            if x == y:
                g[6] += action
            if x + y == 2:
                g[7] += action

        if -3 in g:
            return 'A'
        if 3 in g:
            return 'B'
        if len(moves) < 9:
            return "Pending"
        else:
            return "Draw"


if __name__ == '__main__':
    print(Solution().tictactoe([[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]))
    print(Solution().tictactoe([[0, 0], [2, 0]]))
    print(Solution().tictactoe([[0, 0], [1, 1], [2, 0], [
          1, 0], [1, 2], [2, 1], [0, 1], [0, 2], [2, 2]]))
    print(Solution().tictactoe(
        [[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]))

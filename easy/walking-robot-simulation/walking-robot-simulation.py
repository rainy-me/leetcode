class Solution:

    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        obstacles = set([(x, y) for x, y in obstacles])

        di = 0
        x, y = 0, 0
        ans = 0
        for command in commands:
            if command == -1:
                di = (di + 1) % 4
            elif command == -2:
                di = (di - 1) % 4
            else:
                for _ in range(command):
                    nx, ny = x + directions[di][0], y + directions[di][1]
                    if (nx, ny) not in obstacles:
                        x, y = nx, ny
                        ans = max(ans, x ** 2 + y ** 2)
                    else:
                        break
        return ans


class Solution2:
    def robotSim(self, commands: List[int], obstacles: List[List[int]]) -> int:
        x, y, i, j, d = 0, 0, 0, 0, 0
        res = float('-inf')
        move = [(0, 1), (-1, 0), (0, -1), (1, 0)]
        obs = set(map(tuple, obstacles))
        for command in commands:
            if command == -1:
                d = (d - 1) % 4
            elif command == -2:
                d = (d + 1) % 4
            else:
                i, j = move[d]
                while command and (x+i, y+j) not in obs:
                    x += i
                    y += j
                    command -= 1
            res = max(res, x*x + y*y)
        return res

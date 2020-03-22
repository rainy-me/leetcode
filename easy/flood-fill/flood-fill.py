class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, newColor: int) -> List[List[int]]:
        base = image[sr][sc]
        m, n = len(image), len(image[0])

        def fillCell(x, y):
            if [x, y] in filled:
                return

            image[x][y] = newColor

            nonlocal base
            d = [
                [1, 0],
                [-1, 0],
                [0, 1],
                [0, -1],
            ]
            for (dx, dy) in d:
                next_x, next_y = x+dx, y+dy
                if 0 <= next_x < m and 0 <= next_y < n:
                    v = image[next_x][next_y]
                    if v == base:
                        print(next_x, next_y)
                        fillCell(next_x, next_y)
        fillCell(sr, sc)
        return image

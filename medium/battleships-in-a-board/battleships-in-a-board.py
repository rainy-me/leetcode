class Solution:
    def countBattleships(self, board: List[List[str]]) -> int:
        ans = 0
        in_ship = 0
        for i, row in enumerate(board):
            in_ship = 0
            for j, cell in enumerate(row):
                last_top = False
                if i > 0:
                    try:
                        last_top = board[i-1][j] == 'X'
                    except:
                        pass
                if cell == 'X':
                    if not (in_ship or last_top):
                        print([i, j])
                        ans += 1
                    in_ship = 1
                else:
                    in_ship = 0
        return ans


class Solution2:
    def countBattleships(self, board: List[List[str]]) -> int:
        row = len(board)
        if row == 0:
            return 0
        col = len(board[0])
        cnt = 0

        for i in range(row):
            for j in range(col):
                if (
                    board[i][j] == 'X' and
                    (j == 0 or board[i][j-1] == '.') and
                    (i == 0 or board[i-1][j] == '.')
                ):
                    cnt += 1

        return cnt

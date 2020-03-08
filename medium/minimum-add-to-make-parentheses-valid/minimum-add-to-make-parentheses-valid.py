class Solution:
    def minAddToMakeValid(self, S: str) -> int:
        not_opened = not_closed = 0
        for char in S:
            if not_closed == 0 and char == ')':
                not_opened += 1
            elif char == '(':
                not_closed += 1
            else:
                not_closed -= 1
        return not_opened + not_closed

class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        def getStack(string: str):
            stack = []
            for char in string:
                if char == '#':
                    try:
                        stack.pop()
                    except:
                        pass
                else:
                    stack.append(char)
            return stack
        ss = getStack(S)
        ts = getStack(T)
        if len(ss) != len(ts):
            return False
        for i, c in enumerate(ss):
            if c != ts[i]:
                return False
        return True


class Solution2:
    def backspaceCompare(self, S, T):
        """
        do it backwards
        """
        i, j = len(S) - 1, len(T) - 1
        backS = backT = 0
        while True:
            while i >= 0 and (backS or S[i] == '#'):
                backS += 1 if S[i] == '#' else -1
                i -= 1
            while j >= 0 and (backT or T[j] == '#'):
                backT += 1 if T[j] == '#' else -1
                j -= 1
            if not (i >= 0 and j >= 0 and S[i] == T[j]):
                return i == j == -1
            i, j = i - 1, j - 1

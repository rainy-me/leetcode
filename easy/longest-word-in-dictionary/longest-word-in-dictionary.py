class Solution:

    def __init__(self):
        self.e = []
        self.d = {}

    def add(self, w):
        l = len(w)
        if l == 1:
            self.e.append(w)
            return
        prefix = w[:-1]
        if prefix in self.d:
            self.d[prefix].append(w)
        else:
            self.d[prefix] = [w]

    def find(self, prefix, depth=1):
        if prefix in self.d:
            return self.max([self.find(w, depth+1) for w in self.d[prefix]])
        else:
            return [prefix, depth]

    def max(self, arr):
        _p, _d = "{", 0
        for [p, d] in arr:
            if d > _d:
                _p = p
                _d = d
            elif d == _d and p < _p:
                _p = p
                _d = d
        return [_p, _d]

    def longestWord(self, words: List[str]) -> str:
        for w in words:
            self.add(w)
        return self.max([self.find(e) for e in self.e])[0]

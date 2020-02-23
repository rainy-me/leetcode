from collections import Counter
from typing import List


class Solution:
    def shortestCompletingWord(self, licensePlate: str, words: List[str]) -> str:
        c = Counter([c for c in licensePlate.lower()
                     if c in set('abcdefghijklmnopqrstuvwxyz')])
        _min, _ans = float("inf"), ""
        for w in words:
            wc = Counter(w)
            if len(w) >= _min or any([c[k] > wc.get(k, 0) for k in c]):
                continue
            _ans = w
            _min = len(w)
        return _ans


if __name__ == '__main__':
    print(Solution().shortestCompletingWord("1s3 PSt",
                                            ["step", "steps", "stripe", "stepple"]))
    print(Solution().shortestCompletingWord(
        "1s3 456", ["looks", "pest", "stew", "show"]))
    print(Solution().shortestCompletingWord("Ah71752",
                                            ["suggest", "letter", "of", "husband", "easy",
                                                "education", "drug", "prevent", "writer", "old"]
                                            ))

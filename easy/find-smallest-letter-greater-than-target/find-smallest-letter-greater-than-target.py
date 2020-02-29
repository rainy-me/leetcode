class Solution:
    def nextGreatestLetter(self, letters: List[str], target: str) -> str:
        _ans = "{"
        for l in letters:
            if target < l < _ans:
                _ans = l
        return letters[0] if _ans == '{' else _ans

class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        return [w for w in words if any(w in a for a in words if a != w)]

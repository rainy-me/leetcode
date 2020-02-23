import re


class Solution:
    def countSegments(self, s: str) -> int:
        return len(re.findall(r'\S+', s))

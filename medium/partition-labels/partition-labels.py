class Solution:
    def partitionLabels(self, S: str) -> List[int]:
        lastIndex = {}
        for i, char in enumerate(S):
            lastIndex[char] = i

        current_last = 0
        ans = []
        count = 0
        for i, char in enumerate(S):
            count += 1
            current_last = max(current_last, lastIndex[char])
            if i == current_last:
                ans.append(count)
                count = 0
        return ans

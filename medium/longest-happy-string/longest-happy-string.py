class Solution:
    def longestDiverseString(self, a: int, b: int, c: int) -> str:
        last = ""
        ans = ""
        m = {"a": a, "b": b, "c": c}

        # constract the string with one character
        while True:
            keys = [c for c in m.keys() if c != last]
            char = sorted(keys, key=lambda i: m[i]).pop()
            length = min(m[char], 1)
            update = char * length
            if not update:
                break
            ans += update
            m[char] -= length
            last = char

        # expand by replacing
        for k in m:
            ans = ans.replace(k, k*2, m[k])
        return ans

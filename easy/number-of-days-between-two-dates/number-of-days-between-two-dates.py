from datetime import date


class Solution:
    def daysBetweenDates(self, date1: str, date2: str) -> int:
        s1, s2 = date1.split("-"), date2.split("-")
        d1, d2 = date(*map(int, s1)), date(*map(int, s2))
        return abs((d1-d2).days)

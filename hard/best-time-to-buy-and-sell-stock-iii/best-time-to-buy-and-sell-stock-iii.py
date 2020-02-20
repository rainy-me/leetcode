class Solution(object):
    def maxProfit(self, prices):
        fb, fp, sb, sp = float('inf'), 0, float('inf'), 0
        for price in prices:
            fb = min(fb, price)
            fp = max(fp, price-fb)
            sb = min(sb, price-fp)
            sp = max(sp, price-sb)
            print([[fb, fp], [sb, sp]])
        return sp


if __name__ == '__main__':
    #print(Solution().maxProfit([3, 3, 5, 0, 0, 3, 1, 4]))
    print(Solution().maxProfit([2, 4, 1]))

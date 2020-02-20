import heapq


class Solution(object):
    def maxProfit(self, k, prices):
        n = len(prices)
        if n < 2:
            return 0
        if k == 0:
            return 0
        if k >= n / 2:
            return sum(i - j for i, j in zip(prices[1:], prices[:-1]) if i - j > 0)
        pairs = [[float('inf'), 0] for _ in range(k)]
        for price in prices:
            pairs[0][0] = min(pairs[0][0], price)
            pairs[0][1] = max(pairs[0][1], price-pairs[0][0])

            for i in range(1, len(pairs)):
                pairs[i][0] = min(pairs[i][0], price-pairs[i-1][1])
                pairs[i][1] = max(pairs[i][1], price-pairs[i][0])
        return pairs[-1][1]

# TODO: understand this


class Solution2:
    def maxProfit(self, k, prices):
        length = len(prices)
        v = p = 0
        pairs, profits = [], []

        while p < length:

            v = p
            while v < length - 1 and prices[v] >= prices[v+1]:
                v += 1

            p = v+1
            while p < length and prices[p] >= prices[p-1]:
                p += 1

            while pairs and prices[v] < prices[pairs[-1][0]]:
                heapq.heappush(
                    profits, prices[pairs[-1][0]] - prices[pairs[-1][1] - 1])
                pairs.pop()

            while pairs and prices[p-1] >= prices[pairs[-1][1] - 1]:
                heapq.heappush(profits, prices[v] - prices[pairs[-1][1] - 1])
                v = pairs[-1][0]
                pairs.pop()

            pairs.append((v, p))

        while pairs:
            heapq.heappush(
                profits, prices[pairs[-1][0]] - prices[pairs[-1][1] - 1])
            pairs.pop()

        ans = 0
        while k != 0 and profits:
            ans -= heapq.heappop(profits)
            k -= 1
        return ans


if __name__ == '__main__':
    #print(Solution().maxProfit([3, 3, 5, 0, 0, 3, 1, 4]))
    print(Solution().maxProfit(2, [1, 3, 2, 5, 1, 4]))

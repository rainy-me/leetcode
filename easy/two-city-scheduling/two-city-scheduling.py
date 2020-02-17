from typing import List


class Solution:
    def twoCitySchedCost(self, costs: List[List[int]]) -> int:
        cost, a, b = 0, [], []
        for (ca, cb) in costs:
            if ca < cb:
                a.append(abs(ca - cb))
                cost += ca
            else:
                b.append(abs(ca - cb))
                cost += cb

        if len(a) != len(b):
            la, lb = len(a), len(b)
            if la < lb:
                swap_list = b
            else:
                swap_list = a

            swap_list.sort()
            num_swaps = abs(la - lb)//2
            for i in range(num_swaps):
                cost += swap_list[i]

        return cost

# class Solution:
#    def twoCitySchedCost(self, costs: List[List[int]]) -> int:
#        total = 0
#        costs.sort(key=lambda x: x[0]-x[1])
#        for i in costs[:len(costs)//2]:
#            total += i[0]
#        for j in costs[len(costs)//2:]:
#            total += j[1]
#        return total


if __name__ == '__main__':
    print(Solution().twoCitySchedCost([[259, 770], [448, 54], [926, 667], [184, 139], [840, 118], [577, 469]]
                                      ))

class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.s = []

    def push(self, x: int) -> None:
        self.s.append(x)

    def pop(self) -> None:
        self.s.pop()

    def top(self) -> int:
        return self.s[-1]

    def getMin(self) -> int:
        return min(self.s)


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(x)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()

class MyStack:

    def __init__(self):
        """
        Initialize your data structure here.
        """

        self.q = []

    def push(self, x: int):
        """
        Push element x onto stack.
        """
        self.q = [x, *self.q]

    def pop(self):
        """
        Removes the element on top of the stack and returns that element.
        """
        [x, *self.q] = self.q
        return x

    def top(self):
        """
        Get the top element.
        """
        return self.q[0]

    def empty(self):
        """
        Returns whether the stack is empty.
        """
        return len(self.q) == 0

from typing import List
"""
# Employee info
"""


class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        # It's the unique id of each node.
        # unique id of this employee
        self.id = id
        # the importance value of this employee
        self.importance = importance
        # the id of direct subordinates
        self.subordinates = subordinates


class Solution:
    def getImportance(self, employees: List['Employee'], id: int) -> int:
        m = {}
        for employee in employees:
            m[employee.id] = employee
        this_employee = m[id]

        def calcSubordinates(e: 'Employee'):
            nonlocal m
            im = e.importance
            for s in e.subordinates:
                im += calcSubordinates(m[s])
            return im
        return calcSubordinates(this_employee)

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        head = dummy = ListNode(0)

        while l1 or l2:
            num = 0
            if l1 and l2:
                if l1.val <= l2.val:
                    num = l1.val
                    l1 = l1.next
                else:
                    num = l2.val
                    l2 = l2.next
                head.next = head = ListNode(num)
            elif l1:
                head.next = l1
                return dummy.next
            elif l2:
                head.next = l2
                return dummy.next

        return dummy.next

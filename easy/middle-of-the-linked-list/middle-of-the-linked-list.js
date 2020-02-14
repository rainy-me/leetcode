/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var middleNode = function (head) {
  let half = head;
  let flag = 0;
  while (head.next) {
    head = head.next
    flag ^= 1;
    if (flag & 1) {
      half = half.next
    }
  }
  return half
};
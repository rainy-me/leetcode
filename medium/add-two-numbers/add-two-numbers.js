function arrayToList(array) {
  var list = null;
  for (var i = array.length - 1; i >= 0; i--) {
    list = {
      val: array[i],
      next: list//null, then {value:20, rest: null}
    };
  }
  return list;
}

function ListNode(val) {
  this.val = val;
  this.next = null;
}
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function (l1, l2) {
  let res = new ListNode(-1),
    cursor = res,
    sum = 0, carry = 0;

  while (l1 || l2 || sum > 0) {
    if (l1) {
      sum += l1.val;
      l1 = l1.next;
    }

    if (l2) {
      sum += l2.val;
      l2 = l2.next;
    }

    if (sum >= 10) {
      sum -= 10;
      carry = 1;
    }

    cursor.next = new ListNode(sum);
    cursor = cursor.next;
    sum = carry;
    carry = 0;
  }
  return res.next;
};

console.log(addTwoNumbers(arrayToList([2, 4, 3]), arrayToList([5, 6, 4])))
const assert = require('assert')

function ListNode(val, next) {
  this.val = (val===undefined ? 0 : val)
  this.next = (next===undefined ? null : next)
}

// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDupsOrdered = function(head) {
  let node = head;

  while (node != null) {
      if (node.val == (node.next && node.next.val)) {
          node.next = node.next.next;
      } else {
          node = node.next;
      }
  }

  return head;
};

(function testDelDupOrd() {
  const l1 = new ListNode(1, new ListNode(1, new ListNode(2)));
  assert.deepStrictEqual(
    deleteDupsOrdered(l1),
    new ListNode(1, new ListNode(2))
  );

  const l2 = new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(3)))));
  assert.deepStrictEqual(
    deleteDupsOrdered(l2),
    new ListNode(1, new ListNode(2, new ListNode(3)))
  );
})();

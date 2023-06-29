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

// 2.1 Remove Dups! Write code to remove duplicates from an unsorted linked list.
// FOLLOW UP
// How would you solve this problem if a temporary buffer is not allowed?
// Hints: #9, #40
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDupsUnordered = function(head) {
  const set = new Set();
  let node = head;
  let prev;

  while (node != null) {
    if (set.has(node.val)) {
      prev.next = node.next;
    } else {
      set.add(node.val);
      prev = node;
    }

    node = node.next;
  }

  return head;
};

(function testDelDupUnord() {
  const l1 = new ListNode(2, new ListNode(1, new ListNode(2)));
  assert.deepStrictEqual(
    deleteDupsUnordered(l1),
    new ListNode(2, new ListNode(1))
  );

  const l2 = new ListNode(3, new ListNode(2, new ListNode(3, new ListNode(1, new ListNode(2)))));
  assert.deepStrictEqual(
    deleteDupsUnordered(l2),
    new ListNode(3, new ListNode(2, new ListNode(1)))
  );
})();

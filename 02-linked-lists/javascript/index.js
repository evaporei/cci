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

// buffer not allowed
var dedupUnordNoBuf = function(head) {
  let curr = head;
  while (curr != null) {
    let runner = curr;
    while (runner.next != null) {
      if (runner.next.val == curr.val) {
        runner.next = runner.next.next;
      } else {
        runner = runner.next;
      }
    }
    curr = curr.next;
  }
};

(function testDelDupUnordNoBuf() {
  const l1 = new ListNode(2, new ListNode(1, new ListNode(2)));
  dedupUnordNoBuf(l1);
  assert.deepStrictEqual(
    l1,
    new ListNode(2, new ListNode(1))
  );

  const l2 = new ListNode(3, new ListNode(2, new ListNode(3, new ListNode(1, new ListNode(2)))));
  dedupUnordNoBuf(l2);
  assert.deepStrictEqual(
    l2,
    new ListNode(3, new ListNode(2, new ListNode(1)))
  );
})();

// 2.2 Return Kth to Last: Implement an algorithm to find the kth to last element of a singly linked list.
// Hints: #8, #25, #41, #67, #126
var kthToLast = function(head, k) {
  let len = 0;

  let curr = head;
  while (curr != null) {
    len++;
    curr = curr.next;
  }

  let diff = len - k;
  let kth = head;
  while (diff > 0) {
    kth = kth.next;
    diff--;
  }

  return kth;
};

(function testKthToLast() {
  const l1 = new ListNode(2, new ListNode(1, new ListNode(2)));
  assert.deepStrictEqual(
    kthToLast(l1, 1),
    new ListNode(2)// last element
  );

  const l2 = new ListNode(3, new ListNode(2, new ListNode(3, new ListNode(1, new ListNode(2)))));
  assert.deepStrictEqual(
    kthToLast(l2, 3),
    new ListNode(3, new ListNode(1, new ListNode(2)))// middle element
  );
})();

var kthToLastOpt = function(head, k) {
  let lastPtrs = [];

  let curr = head;
  while (curr != null) {
    lastPtrs.push(curr);
    if (lastPtrs.length > k) {
      lastPtrs.shift();// pop first
    }
    curr = curr.next;
  }

  return lastPtrs.shift();
};

(function testKthToLastOpt() {
  const l1 = new ListNode(2, new ListNode(1, new ListNode(2)));
  assert.deepStrictEqual(
    kthToLastOpt(l1, 1),
    new ListNode(2)// last element
  );

  const l2 = new ListNode(3, new ListNode(2, new ListNode(3, new ListNode(1, new ListNode(2)))));
  assert.deepStrictEqual(
    kthToLastOpt(l2, 3),
    new ListNode(3, new ListNode(1, new ListNode(2)))// middle element
  );
})();

// this places two pointers K nodes apart
// by moving p1 K nodes into the list, then
// walking p2 till the end of p1 (N).
var kthToLastOpt2 = function(head, k) {
  let p1 = head;
  let p2 = head;

  for (let i = 0; i < k; i++) {
    if (p1 == null) return null;
    p1 = p1.next;
  }

  while (p1 != null) {
    p1 = p1.next;
    p2 = p2.next;
  }

  return p2;
};

(function testKthToLastOpt2() {
  const l1 = new ListNode(2, new ListNode(1, new ListNode(2)));
  assert.deepStrictEqual(
    kthToLastOpt2(l1, 1),
    new ListNode(2)// last element
  );

  const l2 = new ListNode(3, new ListNode(2, new ListNode(3, new ListNode(1, new ListNode(2)))));
  assert.deepStrictEqual(
    kthToLastOpt2(l2, 3),
    new ListNode(3, new ListNode(1, new ListNode(2)))// middle element
  );
})();

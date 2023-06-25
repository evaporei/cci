#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
pub fn remove_dups_ordered(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_opt = head.as_mut();

    while let Some(curr) = curr_opt {
        let mut next_opt = curr.next.take();

        while let Some(next) = next_opt.as_mut() {
            if next.val == curr.val {
                next_opt = next.next.take();
            } else {
                curr.next = next_opt;
                break;
            }
        }

        curr_opt = curr.next.as_mut();
    }

    head
}

use std::collections::HashSet;

// 2.1 Remove Dups! Write code to remove duplicates from an unsorted linked list.
// FOLLOW UP
// How would you solve this problem if a temporary buffer is not allowed?
// Hints: #9, #40
pub fn remove_dups_unordered_w_buf(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let head = head?;
    let mut set = HashSet::from([head.val]);
    let mut new_list = ListNode::new(head.val);
    let mut new_head = &mut new_list;
    let mut head = head.next;

    loop {
        match head {
            None => break Some(Box::new(new_list)),
            Some(node) => {
                if !set.contains(&node.val) {
                    set.insert(node.val);
                    new_head.next = Some(Box::new(ListNode::new(node.val)));
                    new_head = &mut *new_list.next.as_mut().unwrap();
                }
                head = node.next;
            }
        }
    }
}

#[test]
fn test_remove_dups() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        })),
    }));

    assert_eq!(remove_dups_ordered(l1), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(2))),
    })));

    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        })),
    }));

    assert_eq!(remove_dups_ordered(l2), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    })));

    // unordered w buf

    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        })),
    }));

    assert_eq!(remove_dups_unordered_w_buf(l1), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(2))),
    })));

    let l2 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(2))),
                })),
            })),
        })),
    }));

    assert_eq!(remove_dups_unordered_w_buf(l2), Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        })),
    })));
}

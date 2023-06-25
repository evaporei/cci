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
}

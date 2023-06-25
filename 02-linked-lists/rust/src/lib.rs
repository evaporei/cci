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

impl FromIterator<i32> for ListNode {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        let mut iter = iter.into_iter();

        let mut head = ListNode::new(iter.next().expect("expected at least one item"));

        let mut tail = &mut head.next;

        while let Some(next) = iter.next() {
            *tail = Some(Box::new(ListNode::new(next)));
            if let Some(tail_next) = tail {
                tail = &mut tail_next.next;
            }
        }

        head
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

pub fn remove_dups_unordered_no_set(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let head = head?;
    let mut new_list = ListNode::new(head.val);
    let mut new_head = &mut new_list;
    let mut pivot: &Option<Box<_>> = &head.next;
    let mut previous_pivot: &Option<Box<_>> = &None;

    loop {
        match pivot {
            None => break Some(Box::new(new_list)),
            Some(node) => {
                if !contains_in_range(node.val, &head, previous_pivot.as_ref().unwrap_or(&head)) {
                    new_head.next = Some(Box::new(ListNode::new(node.val)));
                    new_head = &mut *new_list.next.as_mut().unwrap();
                }
                previous_pivot = pivot;
                pivot = &node.next;
            }
        }
    }
}

fn contains_in_range(val: i32, mut start: &ListNode, end: &ListNode) -> bool {
    loop {
        if start.val == val {
            return true;
        }
        if core::ptr::eq(start, end) {
            return false;
        }
        let Some(ref next) = start.next else {
            return false;
        };
        start = &*next;
    }
}

#[test]
fn test_remove_dups() {
    let l1 = vec![1, 1, 2].into_iter().collect();

    assert_eq!(remove_dups_ordered(Some(Box::new(l1))), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(2))),
    })));

    let l2 = vec![1, 1, 2, 3, 3].into_iter().collect();

    assert_eq!(remove_dups_ordered(Some(Box::new(l2))), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    })));

    // unordered w buf

    let l1 = vec![1, 2, 1].into_iter().collect();

    assert_eq!(remove_dups_unordered_w_buf(Some(Box::new(l1))), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(2))),
    })));

    let l2 = vec![3, 2, 3, 1, 2].into_iter().collect();

    assert_eq!(remove_dups_unordered_w_buf(Some(Box::new(l2))), Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        })),
    })));

    // unordered no set

    let l1 = vec![1, 2, 1].into_iter().collect();

    assert_eq!(remove_dups_unordered_no_set(Some(Box::new(l1))), Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode::new(2))),
    })));

    let l2 = vec![3, 2, 3, 1, 2].into_iter().collect();

    assert_eq!(remove_dups_unordered_no_set(Some(Box::new(l2))), Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        })),
    })));
}

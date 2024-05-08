from linkedlist import LinkedList

# time: O(n)
# space: O(m) m = unique items
def remove_dups_fast(ll: LinkedList[int]):
    s = set()
    curr = ll.head
    prev = None

    while curr is not None:
        if curr.value in s:
            prev.next = curr.next
        else:
            s.add(curr.value)
            prev = curr
        curr = curr.next


# time: O(nˆ2)
# space: O(1)
def remove_dups_small(ll: LinkedList[int]):
    curr = ll.head

    while curr is not None:
        runner = curr
        while runner.next:
            if curr.value == runner.next.value:
                runner.next = runner.next.next
            else:
                runner = runner.next
        curr = curr.next


linked = LinkedList.from_list([5, 5, 3, 6, 2, 6])
remove_dups_fast(linked)
assert linked.to_list() == [5, 3, 6, 2], linked.to_list()

linked = LinkedList.from_list([5, 5, 3, 6, 2, 6])
remove_dups_small(linked)
assert linked.to_list() == [5, 3, 6, 2], linked.to_list()

linked = LinkedList.from_list([5, 3, 5, 6, 2, 6])
remove_dups_small(linked)
assert linked.to_list() == [5, 3, 6, 2], linked.to_list()

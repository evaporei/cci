from linkedlist import LinkedList

def remove_dups(ll: LinkedList[int]):
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


linked = LinkedList.from_list([5, 5, 3, 6, 2, 6])
remove_dups(linked)
assert linked.to_list() == [5, 3, 6, 2], linked.to_list()

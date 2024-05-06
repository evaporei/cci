class Node[T]:
    value: T
    next: "Node | None"

    def __init__(self, value: T):
        self.value = value
        self.next = None


class LinkedList[T]:
    head: Node[T] | None

    def __init__(self, value: T):
        self.head = Node(value)

    # O(N)
    def push(self, value: T):
        curr = self.head
        
        if curr is None:
            curr = Node(value)
            return

        while curr and curr.next is not None:
            curr = curr.next

        curr.next = Node(value)

    # O(N)
    def pop(self) -> Node | None:
        curr = self.head
        prev = curr

        if curr is None:
            return None

        while curr.next is not None:
            prev = curr
            curr = curr.next

        if prev == curr:
            self.head = None
        else:
            prev.next = None

        return curr


linked = LinkedList(1)
linked.push(2)
linked.push(3)
assert linked.pop().value == 3
assert linked.pop().value == 2
assert linked.pop().value == 1
assert linked.pop() is None

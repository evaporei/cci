class Node[T]:
    value: T
    next: "Node | None"

    def __init__(self, value: T):
        self.value = value
        self.next = None


class LinkedList[T]:
    head: Node[T] | None

    def __init__(self, value: T | None = None):
        if value is None:
            self.head = None
        else:
            self.head = Node(value)

    def to_list(self) -> list[T]:
        curr = self.head
        out = []

        while curr is not None:
            out.append(curr.value)
            curr = curr.next

        return out

    @staticmethod
    def from_list(a: list[T]) -> "LinkedList":
        ll = LinkedList()
        for e in a:
            ll.push(e)
        return ll

    # O(N)
    def push(self, value: T):
        curr = self.head
        
        if curr is None:
            self.head = Node(value)
            return

        while curr.next is not None:
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

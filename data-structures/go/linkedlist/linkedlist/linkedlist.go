package linkedlist

type Node[T any] struct {
    value T
    next *Node[T]
}

func new[T any](value T) *Node[T] {
    return &Node[T] {
        value: value,
        next: nil,
    }
}

type LinkedList[T any] struct {
    head *Node[T]
    // to last, so Pop can be O(1)
    prev *Node[T]
    last *Node[T]
}

func New[T any](value T) *LinkedList[T] {
    node := new(value)
    return &LinkedList[T] {
        head: node,
        prev: nil,
        last: node,
    }
}

// O(1)
func (l *LinkedList[T]) Push(value T) {
    l.prev = l.last
    newNode := new(value)
    l.last.next = newNode
    l.last = newNode
}

// O(1)
func (l *LinkedList[T]) Pop() {
    l.last = l.prev
    l.prev.next = nil
}

func (l *LinkedList[T]) Value() T {
    return l.head.value
}

func (l *LinkedList[T]) Next() *LinkedList[T] {
    return &LinkedList[T] {
        head: l.head.next,
        prev: l.prev,
        last: l.last,
    }
}

// O(1)
func (l *LinkedList[T]) Last() *LinkedList[T] {
    return &LinkedList[T] {
        head: l.last,
        prev: nil,
        last: l.last,
    }
}

package main

import (
    "fmt"
    "linkedlist/linkedlist"
)

func main() {
    l := linkedlist.New(2)
    l.Push(4)
    l.Push(6)
    fmt.Println("first", l.Value()) // 2
    fmt.Println("next", l.Next().Value()) // 4
    fmt.Println("last", l.Next().Next().Value()) // 6

    fmt.Println("Pop()")
    l.Pop()
    fmt.Println("first", l.Value()) // 2
    fmt.Println("next", l.Next().Value()) // 4
    fmt.Println("after last", l.Next().Next()) // nil

    fmt.Println("Last", l.Last().Value()) // 4
}

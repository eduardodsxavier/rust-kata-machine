mod stack;
use crate::stack::Stack;

mod queue;
use crate::queue::Queue;

mod double_linked_list;
use crate::double_linked_list::DoubleLinkedList;

fn main() {
    let s: Stack<u32> = Stack::new();
    let dll: DoubleLinkedList<u32> = DoubleLinkedList::new();
    let q: Queue<u32> = Queue::new();

    println!("test stack: {}", test_stack(s));
    println!("test queue: {}", test_queue(q));
    println!("test double linked list: {}", test_dll(dll));
}

fn test_stack(mut s: Stack<u32>) -> bool {
    s.push(1);

    if s.lenght != 1 { return false }

    if s.peek().unwrap() != 1 { return false }

    s.pop();

    if s.lenght != 0 { return false }

    if s.peek().is_some() { return false }

    s.push(1);
    s.push(2);
    s.push(3);

    if s.lenght != 3 { return false }

    if s.peek().unwrap() != 3 { return false }

    s.push(4);
    s.push(5);

    s.pop();

    if s.lenght != 4 { return false }

    if s.peek().unwrap() != 4 { return false }

    return true
}

fn test_queue(mut q: Queue<u32>) -> bool {
    q.enqueue(1);

    if q.lenght != 1 { return false }

    if q.peek().unwrap() != 1 { return false }

    q.dequeue();

    if q.lenght != 0 { return false }

    if q.peek().is_some() { return false }

    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);

    if q.lenght != 3 { return false }

    if q.peek().unwrap() != 2 { return false }

    q.enqueue(5);
    q.enqueue(6);

    q.dequeue();

    if q.lenght != 4 { return false }

    if q.peek().unwrap() != 3 { return false }

    return true
}

fn test_dll(mut dll: DoubleLinkedList<u32>) -> bool {
    dll.prepend(3);

    if dll.lenght != 1 { return false }

    dll.append(5);

    dll.insert_at(1, 4);

    if dll.lenght != 3 { return false }

    if dll.get(0).unwrap() != 3 { return false }

    dll.remove(3);

    if dll.lenght != 2 { return false }

    dll.prepend(2);
    dll.append(6);
    dll.append(7);
    dll.prepend(1);
    dll.append(8);

    dll.remove(5);

    if dll.lenght != 6 { return false }

    dll.remove_at(2);

    if dll.lenght != 5 { return false }

    if dll.get(0).unwrap() != 1 { return false }
    if dll.get(1).unwrap() != 2 { return false }

    return true
}

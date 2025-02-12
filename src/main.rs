mod stack;
use crate::stack::Stack;

mod double_linked_list;
use crate::double_linked_list::DoubleLinkedList;

fn main() {
    let s: Stack<u32> = Stack::new();
    let _dll: DoubleLinkedList<u32> = DoubleLinkedList::new();

    if test_stack(s) {
        print!("working");
        return
    }

    print!("not working");
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

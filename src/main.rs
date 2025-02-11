mod stack;
use crate::stack::Stack;

fn main() {
    let s: Stack<u32> = Stack::new();

    if test_stack(s) {
        print!("working");
        return
    }

    print!("not working");
}

fn test_stack(mut s: Stack<u32>) -> bool {
    s.push(1);

    if s.peek().unwrap() != 1 { return false }

    s.pop();

    if s.peek().is_some() { return false }

    s.push(1);
    s.push(2);
    s.push(3);

    if s.peek().unwrap() != 3 { return false }

    s.push(4);
    s.push(5);

    s.pop();

    if s.peek().unwrap() != 4 { return false }

    return true
}

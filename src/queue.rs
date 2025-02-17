use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

pub struct Queue<T> {
    tail: Option<Rc<RefCell<Node<T>>>>,
    head: Option<Rc<RefCell<Node<T>>>>,
    pub lenght: usize
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { 
            head: None,
            tail: None,
            lenght: 0
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.lenght += 1;

        let node = Rc::new(RefCell::new(Node {value, next:None}));

        if self.tail.is_none() {
            self.tail = Some(Rc::clone(&node));
            self.head = Some(Rc::clone(&node));
            return
        }

        if let Some(tail) = self.tail.clone() {
            let mut tail_borrow = tail.borrow_mut();

            tail_borrow.next = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        }
    }
    
    pub fn dequeue(&mut self) -> Option<T> 
    where 
        T: Clone
    {
        if self.lenght == 0 {
            return None;
        }

        self.lenght -= 1;

        if let Some(head) = self.head.clone() {
            let value = head.borrow().value.clone();

            if let (Some(head_rc), Some(tail_rc)) = (&self.head, &self.tail) {
                if Rc::ptr_eq(head_rc, tail_rc) {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.head = head.borrow_mut().next.clone();
            return Some(value);
        }
        return None;
    }

    pub fn peek(&self) -> Option<T> 
    where 
        T: Copy, 
    {
        if let Some(head) = self.head.clone() {
            let head_borrow = head.borrow_mut();
            return Some(head_borrow.value)
        }
        None
    }
}

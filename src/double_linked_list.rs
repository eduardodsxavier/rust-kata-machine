use std::rc::Rc;
use std::cell::RefCell;


struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>> 
}

pub struct DoubleLinkedList<T> {
    lenght: usize,
    head: Option<Rc<RefCell<Node<T>>>>, 
    tail: Option<Rc<RefCell<Node<T>>>> 
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> DoubleLinkedList<T> {
        DoubleLinkedList {
            lenght: 0,
            head: None,
            tail: None
        }
    }

    fn prepend(&mut self, value: T) {
        self.lenght += 1;
        let node = Rc::new(RefCell::new(Node {value, prev: None, next:None}));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
            return;
        }

        let head = self.head.take().unwrap();
        head.borrow_mut().prev = Some(Rc::clone(&node));
        node.borrow_mut().next = Some(Rc::clone(&head));
        self.head = Some(Rc::clone(&node));
    }

    fn insert_at(&mut self) {}

    fn append(&mut self, value: T) {
        self.lenght += 1;
        let node = Rc::new(RefCell::new(Node {value, prev: None, next:None}));

        if self.tail.is_none() {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
            return;
        }

        let tail = self.tail.take().unwrap();
        tail.borrow_mut().next = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::clone(&tail));
        self.tail = Some(Rc::clone(&node));
    }

    fn remove(&mut self) {}

    fn get(&mut self, index: usize) {
        if index < 0 || index >= self.lenght {
            return
        }
        let mut node_p: Option<Rc<RefCell<Node<T>>>> = self.head;
        for _ in 0..index {
            todo!()
        }

    }

    fn remove_at(&mut self) {}

}

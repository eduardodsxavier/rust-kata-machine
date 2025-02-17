use std::rc::Rc;
use std::cell::RefCell;
use std::usize;


struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>> 
}

pub struct DoubleLinkedList<T> {
    pub lenght: usize,
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

    pub fn prepend(&mut self, value: T) {
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

    pub fn insert_at(&mut self, index: usize, value: T) {
        if index == 0 {
            self.prepend(value);
            return
        }

        if index == self.lenght {
            self.append(value);
            return 
        }

        if index > self.lenght {
            return
        }

        self.lenght += 1;

        let node = Rc::new(RefCell::new(Node {value, prev: None, next:None}));

        let mut node_p: Option<Rc<RefCell<Node<T>>>> = Some(Rc::clone(&self.head.clone().unwrap()));
        for _ in 0..index {
            node_p = Some(Rc::clone(&node_p.unwrap().borrow_mut().next.clone().unwrap()));
        }

        if node_p.is_none() {
            return
        }

        if let Some(node_p) = node_p.clone() {
            let mut node_borrow = node.borrow_mut();

            node_borrow.prev = Some(Rc::clone(&node_p));

            if let Some(node_p_next) = node_p.borrow_mut().next.clone() {
                node_borrow.next = Some(Rc::clone(&node_p_next));

                node_p_next.borrow_mut().prev = Some(Rc::clone(&node));
            }

            node_p.borrow_mut().next = Some(Rc::clone(&node));
        }

    }

    pub fn append(&mut self, value: T) {
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

    pub fn remove(&mut self, value: T) -> Option<T> 
    where 
        T: PartialEq + Clone
    {
        let mut node_p: Option<Rc<RefCell<Node<T>>>> = Some(Rc::clone(&self.head.clone().unwrap()));
        for _ in 0..self.lenght {
            if node_p.clone().unwrap().borrow_mut().value == value {
                break;
            }
            node_p = Some(Rc::clone(&node_p.unwrap().borrow_mut().next.clone().unwrap()));
        }

        if node_p.is_none() {
            return None
        }

        self.lenght -= 1;

        let value = node_p.clone().unwrap().borrow_mut().value.clone();

        if node_p.clone().unwrap().borrow_mut().prev.is_none() {
            if node_p.clone().unwrap().borrow_mut().next.is_none() {
                self.head = None;
                self.tail = None;
                return Some(value)
            }
            self.head = node_p.clone().unwrap().borrow_mut().next.clone();
            node_p.clone().unwrap().borrow_mut().next.clone().unwrap().borrow_mut().prev = None;
            return Some(value)
        }

        if node_p.clone().unwrap().borrow_mut().next.is_none() {
            node_p.clone().unwrap().borrow_mut().prev.clone().unwrap().borrow_mut().next = None;
            self.tail = node_p.clone().unwrap().borrow_mut().prev.clone();
        }

        if let Some(node_rc) = node_p.clone() {
            let node = node_rc.borrow_mut(); // Get mutable borrow once

            if let Some(next_rc) = node.next.clone() {
                let mut next = next_rc.borrow_mut(); // Get mutable borrow for next
                next.prev = node.prev.clone();
            }

            if let Some(prev_rc) = node.prev.clone() {
                let mut prev = prev_rc.borrow_mut(); // Get mutable borrow for prev
                prev.next = node.next.clone();
            }
        }

        return Some(value)
    }

    pub fn get(&mut self, index: usize) -> Option<T>
    where 
        T: Clone
    {
        if index >= self.lenght {
            return None
        }
        let mut node_p: Option<Rc<RefCell<Node<T>>>> = Some(Rc::clone(&self.head.clone().unwrap()));
        for _ in 0..index {
            node_p = Some(Rc::clone(&node_p.unwrap().borrow_mut().next.clone().unwrap()));
        }

        Some(node_p.unwrap().borrow().value.clone())  
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T>
    where 
        T: Clone
    {
        if index >= self.lenght {
            return None
        }

        let mut node_p: Option<Rc<RefCell<Node<T>>>> = Some(Rc::clone(&self.head.clone().unwrap()));
        for _ in 0..index {
            node_p = Some(Rc::clone(&node_p.unwrap().borrow_mut().next.clone().unwrap()));
        }

        if node_p.is_none() {
            return None
        }

        self.lenght -= 1;

        let value = node_p.clone().unwrap().borrow_mut().value.clone();

        if node_p.clone().unwrap().borrow_mut().prev.is_none() {
            if node_p.clone().unwrap().borrow_mut().next.is_none() {
                self.head = None;
                self.tail = None;
                return Some(value)
            }
            self.head = node_p.clone().unwrap().borrow_mut().next.clone();
            node_p.clone().unwrap().borrow_mut().next.clone().unwrap().borrow_mut().prev = None;
            return Some(value)
        }

        if node_p.clone().unwrap().borrow_mut().next.is_none() {
            node_p.clone().unwrap().borrow_mut().prev.clone().unwrap().borrow_mut().next = None;
            self.tail = node_p.clone().unwrap().borrow_mut().prev.clone();
        }

        if let Some(node_rc) = node_p.clone() {
            let node = node_rc.borrow_mut();

            if let Some(next_rc) = node.next.clone() {
                let mut next = next_rc.borrow_mut();
                next.prev = node.prev.clone();
            }

            if let Some(prev_rc) = node.prev.clone() {
                let mut prev = prev_rc.borrow_mut();
                prev.next = node.next.clone();
            }
        }

        return Some(value)
    }
}

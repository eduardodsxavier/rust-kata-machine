use std::rc::Rc;
use std::cell::RefCell;
use std::usize;


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

    fn insert_at(&mut self, index: usize, value: T) {
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

        node.borrow_mut().prev = Some(Rc::clone(&node_p.clone().unwrap()));
        node.borrow_mut().prev.clone().unwrap().borrow_mut().next = Some(Rc::clone(&node.clone()));

        node.borrow_mut().next = Some(Rc::clone(&node_p.clone().unwrap().borrow_mut().next.clone().unwrap()));
        node.borrow_mut().next.clone().unwrap().borrow_mut().prev = Some(Rc::clone(&node));
    }

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

    fn remove(&mut self, value: T) -> Option<T> 
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


        node_p.clone().unwrap().borrow_mut().next.clone().unwrap().borrow_mut().prev =
            node_p.clone().unwrap().borrow_mut().prev.clone();
        node_p.unwrap().borrow_mut().prev.clone().unwrap().borrow_mut().next =
            node_p.clone().unwrap().borrow_mut().next.clone();

        return Some(value)
    }

    fn get(&mut self, index: usize) -> Option<T>
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

    fn remove_at(&mut self) {}

}

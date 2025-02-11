struct Node<T> {
    value: T,
    previous: Option<Box<Node<T>>>
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    pub lenght: usize
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { 
            top: None,
            lenght: 0
        }
    }

    pub fn push(&mut self, value: T) {
        self.lenght += 1;

        let mut node = Box::new(Node {value, previous: None} );

        if self.top.is_none() {
            self.top = Some(node);
            return
        }

        node.previous = self.top.take();
        self.top = Some(node);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.top.is_none() {
            return None
        }

        self.lenght -= 1;

        let node = self.top.take().unwrap();
        let value = node.value;
        self.top = node.previous;
        return Some(value)
    }

    pub fn peek(&self) -> Option<T> 
    where 
        T: Copy, 
    {
        self.top.as_ref().map(|node| node.value)
    }

}


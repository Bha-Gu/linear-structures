use std::{cell::RefCell, rc::Rc};

struct Node<T: Clone> {
    value: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Stack<T: Clone> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Stack<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, item: T) {
        if let Some(head) = self.head.clone() {
            let node = Rc::new(RefCell::new(Node {
                value: item,
                prev: Some(head),
            }));
            self.head = Some(node);
        } else {
            let node = Rc::new(RefCell::new(Node {
                value: item,
                prev: None,
            }));
            self.head = Some(node);
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let out = self.head.clone()?.borrow().value.clone();
        self.head = self.head.clone()?.borrow().prev.clone();
        self.length -= 1;
        Some(out)
    }

    pub fn peek(&self) -> Option<T> {
        Some(self.head.clone()?.borrow().value.clone())
    }
}

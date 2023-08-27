use std::{cell::RefCell, rc::Rc};

struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct Queue<T: Clone> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Queue<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub const fn len(&self) -> usize {
        self.length
    }
    pub fn enqueue(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
        }));
        if self.length == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node);
            self.length = 1;
        } else if let Some(tail) = self.tail.clone() {
            self.tail = Some(node.clone());
            tail.borrow_mut().next = Some(node);
            self.length += 1;
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        if let Some(head) = self.head.clone() {
            self.head = head.borrow().next.clone();
            self.length -= 1;
            if self.length == 0 {
                self.tail = None;
            }
            Some(head.borrow().value.clone())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<T> {
        Some(self.head.clone()?.borrow().value.clone())
    }
}

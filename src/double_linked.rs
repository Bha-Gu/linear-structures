use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct DLList<T: Clone> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> DLList<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn prepend(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: self.head.clone(),
            prev: None,
        }));
        self.length += 1;
        if let Some(head) = self.head.clone() {
            head.borrow_mut().prev = Some(node.clone());
        }
        self.head = Some(node);
    }
}

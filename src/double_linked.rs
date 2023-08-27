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

impl<T: Clone> Drop for DLList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() && self.unprepend().is_some() {}
    }
}

impl<T: Clone> DLList<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn add_first_node(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
            prev: None,
        }));
        self.length = 1;
        self.head = Some(node.clone());
        self.tail = Some(node);
    }

    pub fn prepend(&mut self, item: T) {
        if self.length == 0 {
            self.add_first_node(item);
            return;
        }
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

    pub fn append(&mut self, item: T) {
        if self.length == 0 {
            self.add_first_node(item);
            return;
        }
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
            prev: self.tail.clone(),
        }));
        self.length += 1;
        if let Some(head) = self.tail.clone() {
            head.borrow_mut().next = Some(node.clone());
        }
        self.tail = Some(node);
    }

    fn remove_last_node(&mut self) -> Option<T> {
        let out = self.head.clone()?.borrow().value.clone();
        self.head = None;
        self.tail = None;
        self.length = 0;
        Some(out)
    }

    pub fn unprepend(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            1 => self.remove_last_node(),
            _ => {
                let out = self.head.clone()?.borrow().value.clone();
                self.head = self.head.clone()?.borrow().next.clone();
                self.length -= 1;
                Some(out)
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            1 => self.remove_last_node(),
            _ => {
                let out = self.tail.clone()?.borrow().value.clone();
                self.tail = self.tail.clone()?.borrow().prev.clone();
                self.length -= 1;
                Some(out)
            }
        }
    }
}

use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Node<T: Clone> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T: Clone> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    pub fn prepend(&mut self, item: T) {
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: self.head.clone(),
        }));
        self.length += 1;
        self.head = Some(node);
    }

    pub fn append(&mut self, item: T) {
        self.length += 1;
        let mut curr = self.head.clone();
        if curr.is_none() {
            self.prepend(item);
            return;
        }
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
        }));

        while let Some(c_node) = curr {
            if c_node.borrow().next.is_some() {
                curr = c_node.borrow().next.clone();
            } else {
                c_node.borrow_mut().next = Some(node);
                break;
            }
        }
    }

    pub fn insert_at(&mut self, item: T, idx: usize) {
        if idx == 0 || self.length == 0 {
            self.prepend(item);
        } else if idx >= self.length {
            self.append(item);
        } else {
            self.length += 1;
            let mut curr = self.head.clone();
            for _i in 0..(idx - 1) {
                if let Some(c_node) = curr {
                    curr = c_node.borrow().next.clone();
                } else {
                    break;
                }
            }
            let c_node = curr.unwrap();
            let node = Rc::new(RefCell::new(Node {
                value: item,
                next: c_node.borrow().next.clone(),
            }));
            c_node.borrow_mut().next = Some(node);
        }
    }

    pub fn copy_to_vec(&self) -> Vec<T> {
        let mut out = vec![];
        let mut curr = self.head.clone();
        while let Some(c_node) = curr {
            out.push(c_node.borrow().value.clone());
            curr = c_node.borrow().next.clone();
        }
        out
    }
}

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

    pub const fn len(&self) -> usize {
        self.length
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
        if let Some(tail) = self.tail.clone() {
            tail.borrow_mut().next = Some(node.clone());
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
                self.head.clone()?.borrow_mut().prev = None;
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
                self.tail.clone()?.borrow_mut().next = None;
                self.length -= 1;
                Some(out)
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
                if let Some(ref_node) = curr {
                    curr = ref_node.borrow().next.clone();
                } else {
                    break;
                }
            }
            let ref_node =
                curr.expect("Should not be possible to see.\n fn insert_at index out of bound");
            let node = Rc::new(RefCell::new(Node {
                value: item,
                next: ref_node.borrow().next.clone(),
                prev: Some(ref_node.clone()),
            }));
            ref_node.borrow().next.clone().unwrap().borrow_mut().prev = Some(node.clone());
            ref_node.borrow_mut().next = Some(node);
        }
    }

    pub fn remove_at(&mut self, idx: usize) -> Option<T> {
        if self.length == 1 {
            self.remove_last_node()
        } else if idx == 0 {
            self.unprepend()
        } else if idx + 1 >= self.length {
            self.pop()
        } else {
            let mut curr = self.head.clone();
            for _i in 0..(idx - 1) {
                if let Some(ref_node) = curr {
                    curr = ref_node.borrow().next.clone();
                } else {
                    break;
                }
            }
            let ref_node = curr?;
            let next = ref_node.borrow().next.clone()?.borrow().next.clone();
            let prev = ref_node.borrow().next.clone()?.borrow().prev.clone();
            let out = ref_node.borrow().next.clone()?.borrow().value.clone();
            ref_node.borrow_mut().next = next.clone();
            next?.borrow_mut().prev = prev;
            self.length -= 1;
            Some(out)
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

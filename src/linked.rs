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
        let mut curr = self.head.clone();
        if curr.is_none() {
            self.prepend(item);
            return;
        }
        self.length += 1;
        let node = Rc::new(RefCell::new(Node {
            value: item,
            next: None,
        }));

        while let Some(ref_node) = curr {
            if ref_node.borrow().next.is_some() {
                curr = ref_node.borrow().next.clone(); //borrow => read only refered var
            } else {
                ref_node.borrow_mut().next = Some(node); // borrow_mut => modift refered var
                break;
            }
        }
    }

    pub fn unprepend(&mut self) {
        if self.length > 0 {
            self.length -= 1;
            self.head = self.head.clone().unwrap().borrow().next.clone();
        }
    }

    pub fn pop(&mut self) {
        if self.length <= 1 {
            self.unprepend();
        } else {
            let mut curr = self.head.clone();
            if self.length == 2 {
                curr.unwrap().borrow_mut().next = None;
            } else {
                for _ in 0..self.length - 2 {
                    if let Some(ref_node) = curr {
                        curr = ref_node.borrow().next.clone();
                    }
                }
                curr.expect("Should not be possible\n fn pop over steped")
                    .borrow_mut()
                    .next = None;
            }
            // ref_node.borrow_mut().next = None;
            self.length -= 1;
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
            }));
            ref_node.borrow_mut().next = Some(node);
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

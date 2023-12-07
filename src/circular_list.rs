use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T: Copy> {
    pub n: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    #[allow(dead_code)]
    pub fn new_rc_node(n: T) -> Rc<RefCell<Node<T>>> {
        let node = Rc::new(RefCell::new(Node { n, next: None }));
        node.borrow_mut().next = Some(node.clone());
        node
    }
}

pub trait CircularList<T: Copy> {
    fn next(&self) -> Option<Rc<RefCell<Node<T>>>>;
    fn append(&self, next: Rc<RefCell<Node<T>>>);
    fn pushn(&self, next: Rc<RefCell<Node<T>>>, n: usize);
    fn popn(&self, n: usize) -> Rc<RefCell<Node<T>>>;
    fn get_n_values(&self, n: usize) -> Vec<T>;
    fn set_n_values(&self, n: usize, vec: &mut Vec<T>);
}

impl<T: Copy> CircularList<T> for Rc<RefCell<Node<T>>> {
    fn next(&self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.borrow().next.is_none() {
            None
        } else {
            Some(self.borrow().next.as_ref().unwrap().clone())
        }
    }

    fn append(&self, next: Rc<RefCell<Node<T>>>) {
        next.borrow_mut().next = self.borrow_mut().next.take();
        self.borrow_mut().next = Some(next);
    }

    fn pushn(&self, next: Rc<RefCell<Node<T>>>, n: usize) {
        let mut tail: Rc<RefCell<Node<T>>> = next.clone();

        for _ in 0..n - 1 {
            tail = tail.clone().borrow().next.as_ref().unwrap().clone();
        }
        tail.borrow_mut().next = self.borrow_mut().next.take();
        self.borrow_mut().next = Some(next);
    }

    fn popn(&self, n: usize) -> Rc<RefCell<Node<T>>> {
        let mut next = self.clone().borrow().next.as_ref().unwrap().clone();
        let taken = next.clone();

        for _ in 0..n {
            next = next.clone().borrow().next.as_ref().unwrap().clone();
        }
        self.borrow_mut().next = Some(next);

        taken
    }

    fn get_n_values(&self, n: usize) -> Vec<T> {
        let mut values = vec![];
        let mut curr = self.clone();

        for _ in 0..n {
            values.push(curr.borrow().n);
            curr = curr.clone().borrow().next.as_ref().unwrap().clone();
        }

        values
    }

    fn set_n_values(&self, n: usize, vec: &mut Vec<T>) {
        let mut curr = self.clone();

        for i in 0..n {
            vec[i] = curr.borrow().n;
            curr = curr.clone().borrow().next.as_ref().unwrap().clone();
        }
    }
}

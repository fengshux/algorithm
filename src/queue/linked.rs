
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node{val:v,next:None}
    }
}

pub struct CircleQueue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> CircleQueue<T> where T: Clone { 
    pub fn new() -> CircleQueue<T> {
        CircleQueue{head:None,tail:None}
    }

    pub fn get_first(&mut self) -> Option<T> {
        match self.head.clone() {
            None => None,
            Some(head) => {
                self.head = head.borrow_mut().next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(head.borrow().val.clone())
            }
        }
    }

    pub fn append_tail(&mut self,  v: T) {
        let node = Rc::new(RefCell::new(Node::new(v)));
        if self.head.is_none() {
            self.head = Some(node.clone());
        } else {
            let tail = self.tail.clone().unwrap();
            tail.borrow_mut().next = Some(node.clone());
        }
        self.tail = Some(node);
    }    
}

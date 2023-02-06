use std::rc::Rc;
use std::cell::{RefCell};
use std::sync::{Arc, Mutex};

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, tail: None }
    }

    fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct SafeList<T> {
    list: Arc<Mutex<List<T>>>
}
impl <T> SafeList<T> {
    pub fn new() -> Self {
        Self { list: Arc::new(Mutex::new(List::new())) }
    }
    pub fn push_front(&self, elem: T) {
        self.list.lock().unwrap().push_front(elem);
    }
    pub fn push_back(&self, elem: T) {
        self.list.lock().unwrap().push_back(elem);
    }
    pub fn pop_front(&self) -> Option<T> {
        self.list.lock().unwrap().pop_front()
    }
    pub fn pop_back(&self) -> Option<T> {
        self.list.lock().unwrap().pop_back()
    }
}

impl <T> Clone for SafeList<T> {
    fn clone(&self) -> Self {
        SafeList { list: self.list.clone() }
    }
}
unsafe impl <T> Send for List<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_thread() {
        let list: SafeList<i32> = SafeList::new();
        list.push_back(1);
        list.push_back(0);
        assert!(list.pop_front().unwrap() == 1);
        list.push_front(2);
        assert!(list.pop_front().unwrap() == 2);
        _ = list.pop_back();
        assert!(list.pop_front() == None);
    }
}
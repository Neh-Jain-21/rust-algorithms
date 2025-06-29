use std::{cell::RefCell, ptr, rc::Rc};

use crate::utils::comparator::Comparator;
use linked_list_node::{Link, LinkedListNode};

#[path = "./linked_list_node.rs"]
mod linked_list_node;

pub struct LinkedList<T>
where
    T: PartialEq + PartialOrd + 'static + Clone + std::fmt::Debug,
{
    head: Link<T>,
    tail: Link<T>,
    compare: Comparator<T>,
}

impl<T> LinkedList<T>
where
    T: PartialEq + PartialOrd + Clone + std::fmt::Debug,
{
    pub fn new(compare_function: Option<Box<dyn Fn(&T, &T) -> i32>>) -> Self {
        LinkedList {
            head: None,
            tail: None,
            compare: Comparator::new(compare_function),
        }
    }

    pub fn prepend(&mut self, value: T) -> &mut Self {
        let new_node: Rc<RefCell<LinkedListNode<T>>> = LinkedListNode::new(value, self.head.take());

        self.head = Some(new_node.clone());

        if self.tail.is_none() {
            self.tail = Some(new_node);
        }

        return self;
    }

    pub fn append(&mut self, value: T) -> &mut Self {
        let new_node: Rc<RefCell<LinkedListNode<T>>> = LinkedListNode::new(value, None);

        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);

            return self;
        }

        if let Some(tail) = self.tail.clone() {
            tail.borrow_mut().next = Some(new_node.clone());
        }
        self.tail = Some(new_node);

        return self;
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.prepend(value);
        } else {
            let mut count: usize = 0;
            let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

            while let Some(ref node) = current {
                if let Ok(node) = Rc::<RefCell<LinkedListNode<T>>>::try_unwrap(node.clone()) {
                    if count == index - 1 {
                        let new_node: Rc<RefCell<LinkedListNode<T>>> =
                            LinkedListNode::new(value, node.borrow_mut().next.take());

                        node.borrow_mut().next = Some(new_node.clone());

                        if new_node.borrow().next.is_none() {
                            self.tail = Some(new_node);
                        }

                        break;
                    }

                    current = node.borrow_mut().next.clone();
                    count += 1;
                }
            }
        }
    }

    pub fn delete(&mut self, value: T) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        let mut deleted_node: Option<Rc<RefCell<LinkedListNode<T>>>> = None;

        while let Some(head) = self.head.clone() {
            if let Ok(node) = Rc::<RefCell<LinkedListNode<T>>>::try_unwrap(head.clone()) {
                if self.compare.equal(&node.borrow().value, &value) {
                    deleted_node = self.head.clone();
                    self.head = node.borrow_mut().next.take();
                } else {
                    break;
                }
            }
        }

        let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

        while let Some(ref mut node) = current {
            if let Ok(node) = Rc::<RefCell<LinkedListNode<T>>>::try_unwrap(node.clone()) {
                if let Some(next) = node.borrow().next.clone() {
                    if self.compare.equal(&next.borrow().value, &value) {
                        node.borrow_mut().next = next.borrow_mut().next.take();
                        deleted_node = Some(next);
                    } else {
                        current = Some(next);
                    }
                }
            }
        }

        if let Some(tail) = self.tail.clone() {
            if let Ok(tail) = Rc::<RefCell<LinkedListNode<T>>>::try_unwrap(tail.clone()) {
                if self.compare.equal(&tail.borrow().value, &value) {
                    self.tail = current;
                }
            }
        }

        deleted_node
    }

    pub fn find(
        &self,
        value: Option<&T>,
        callback: Option<&dyn Fn(&T) -> bool>,
    ) -> Option<LinkedListNode<T>> {
        let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

        while let Some(ref node) = current {
            if let Ok(node) = Rc::<RefCell<LinkedListNode<T>>>::try_unwrap(node.clone()) {
                if let Some(cb) = callback {
                    if cb(&node.borrow().value) {
                        return Some(node.borrow().clone());
                    }
                } else if let Some(v) = value {
                    if self.compare.equal(&node.borrow().value, v) {
                        return Some(node.borrow().clone());
                    }
                }

                current = node.borrow().next.clone();
            }
        }

        None
    }

    pub fn delete_tail(&mut self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        if self.head.is_none() {
            return None;
        }

        let deleted_tail: Option<Rc<RefCell<LinkedListNode<T>>>> = self.tail.clone();

        if ptr::eq(self.head.as_ref().unwrap(), self.tail.as_ref().unwrap()) {
            self.head = None;
            self.tail = None;
        } else {
            let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

            while let Some(node) = current {
                if let Some(next) = node.borrow().next.clone() {
                    if ptr::eq(&next, self.tail.as_ref().unwrap()) {
                        node.borrow_mut().next = None;
                        self.tail = Some(node.clone());
                        break;
                    }
                }
                current = node.borrow().next.clone();
            }
        }

        deleted_tail
    }

    pub fn delete_head(&mut self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        if let Some(head) = self.head.clone() {
            self.head = head.borrow_mut().next.take();
            if self.head.is_none() {
                self.tail = None;
            }
            return Some(head);
        }

        None
    }

    pub fn from_vec(&mut self, values: Vec<T>) {
        for value in values {
            self.append(value);
        }
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut nodes: Vec<T> = Vec::new();
        let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

        while let Some(node) = current {
            nodes.push(node.borrow().clone().value);
            current = node.borrow().next.clone();
        }

        nodes
    }

    pub fn to_string(&self) -> String {
        let mut nodes: Vec<String> = Vec::new();
        let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

        while let Some(node) = current {
            nodes.push(format!("{:?}", node.borrow().value));
            current = node.borrow().next.clone();
        }

        nodes.join(",")
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Rc<RefCell<LinkedListNode<T>>>> = None;
        let mut current: Option<Rc<RefCell<LinkedListNode<T>>>> = self.head.clone();

        self.tail = self.head.clone();

        while let Some(node) = current {
            let next: Option<Rc<RefCell<LinkedListNode<T>>>> = node.borrow_mut().next.take();

            node.borrow_mut().next = prev;

            prev = Some(node.clone());

            current = next;
        }

        self.head = prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_linked_list() {
        let list: LinkedList<i32> = LinkedList::new(None);
        assert_eq!(list.to_string(), "");
    }

    #[test]
    fn should_append_node_to_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new(None);

        assert_eq!(list.head.is_none(), true);
        assert_eq!(list.tail.is_none(), true);

        list.append(1);
        list.append(2);

        assert_eq!(list.to_string(), "1,2");
        assert_eq!(list.tail.unwrap().borrow().next.is_none(), true);
    }

    #[test]
    fn should_prepend_node_to_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new(None);

        list.prepend(2);

        assert_eq!(list.head.as_ref().unwrap().borrow().value, 2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);

        list.append(1);
        list.prepend(3);

        assert_eq!(list.to_string(), "3,2,1");
    }

    #[test]
    fn should_insert_node_to_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new(None);

        list.insert(4, 3);

        assert_eq!(list.head.as_ref().unwrap().borrow().value, 4);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 4);

        list.insert(3, 2);
        list.insert(2, 1);
        list.insert(10, 9);

        println!("{}", list.to_string());

        assert_eq!(list.to_string(), "2,3,4,10");
    }
}

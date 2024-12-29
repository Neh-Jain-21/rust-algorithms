use std::ptr;

use crate::utils::comparator::Comparator;
use linked_list_node::{Link, LinkedListNode};

#[path = "./linked_list_node.rs"]
mod linked_list_node;

pub struct LinkedList<T>
where
    T: PartialEq + PartialOrd + 'static + Clone,
{
    head: Link<T>,
    tail: Link<T>,
    compare: Comparator<T>,
}

impl<T> LinkedList<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    pub fn new(compare_function: Option<Box<dyn Fn(&T, &T) -> i32>>) -> Self {
        LinkedList {
            head: None,
            tail: None,
            compare: Comparator::new(compare_function),
        }
    }

    pub fn prepend(&mut self, value: T) {
        let new_node: Box<LinkedListNode<T>> = LinkedListNode::new(value, self.head.take());

        self.head = Some(new_node.clone());

        if self.tail.is_none() {
            self.tail = Some(new_node);
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node: Box<LinkedListNode<T>> = LinkedListNode::new(value, None);

        if self.head.is_none() {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(new_node);
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.prepend(value);
        } else {
            let mut count: usize = 0;
            let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();

            while let Some(mut node) = current {
                if count == index - 1 {
                    let new_node: Box<LinkedListNode<T>> =
                        LinkedListNode::new(value, node.next.take());

                    node.next = Some(new_node.clone());

                    if new_node.next.is_none() {
                        self.tail = Some(new_node);
                    }

                    break;
                }

                current = node.next.clone();
                count += 1;
            }
        }
    }

    pub fn delete(&mut self, value: T) -> Option<Box<LinkedListNode<T>>> {
        let mut deleted_node: Option<Box<LinkedListNode<T>>> = None;

        while let Some(mut head) = self.head.clone() {
            if self.compare.equal(&head.value, &value) {
                deleted_node = self.head.clone();
                self.head = head.next.take();
            } else {
                break;
            }
        }

        let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();

        while let Some(ref mut node) = current {
            if let Some(mut next) = node.next.clone() {
                if self.compare.equal(&next.value, &value) {
                    node.next = next.next.take();
                    deleted_node = Some(next);
                } else {
                    current = Some(next);
                }
            } else {
                break;
            }
        }

        if let Some(tail) = self.tail.clone() {
            if self.compare.equal(&tail.value, &value) {
                self.tail = current;
            }
        }

        deleted_node
    }

    pub fn find(
        &self,
        value: Option<&T>,
        callback: Option<&dyn Fn(&T) -> bool>,
    ) -> Option<Box<LinkedListNode<T>>> {
        let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();

        while let Some(node) = current {
            if let Some(cb) = callback {
                if cb(&node.value) {
                    return Some(node);
                }
            } else if let Some(v) = value {
                if self.compare.equal(&node.value, v) {
                    return Some(node);
                }
            }

            current = node.next.clone();
        }

        None
    }

    pub fn delete_tail(&mut self) -> Option<Box<LinkedListNode<T>>> {
        if self.head.is_none() {
            return None;
        }

        let deleted_tail: Option<Box<LinkedListNode<T>>> = self.tail.clone();

        if ptr::eq(self.head.as_ref().unwrap(), self.tail.as_ref().unwrap()) {
            self.head = None;
            self.tail = None;
        } else {
            let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();
            while let Some(mut node) = current {
                if let Some(next) = node.next.clone() {
                    if ptr::eq(&next, self.tail.as_ref().unwrap()) {
                        node.next = None;
                        self.tail = Some(node);
                        break;
                    }
                }
                current = node.next.clone();
            }
        }

        deleted_tail
    }

    pub fn delete_head(&mut self) -> Option<Box<LinkedListNode<T>>> {
        if let Some(mut head) = self.head.clone() {
            self.head = head.next.take();
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
        let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();
        while let Some(node) = current {
            nodes.push(node.value.clone());
            current = node.next.clone();
        }
        nodes
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<LinkedListNode<T>>> = None;
        let mut current: Option<Box<LinkedListNode<T>>> = self.head.clone();
        self.tail = self.head.clone();
        while let Some(mut node) = current {
            let next: Option<Box<LinkedListNode<T>>> = node.next.take();
            node.next = prev;
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
    fn test_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new(None);

        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.to_vec(), vec![1, 2, 3]);

        // list.prepend(0);
        // assert_eq!(list.to_vec(), vec![0, 1, 2, 3]);

        // list.delete(2);
        // assert_eq!(list.to_vec(), vec![0, 1, 3]);

        // list.reverse();
        // assert_eq!(list.to_vec(), vec![3, 1, 0]);

        // list.insert(2, 1);
        // assert_eq!(list.to_vec(), vec![3, 2, 1, 0]);

        // assert_eq!(list.delete_head().unwrap().value, 3);
        // assert_eq!(list.delete_tail().unwrap().value, 0);
        // assert_eq!(list.to_vec(), vec![2, 1]);
    }
}

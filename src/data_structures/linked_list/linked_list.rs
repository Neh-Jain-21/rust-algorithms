use linked_list_node::Link;

#[path = "./linked_list_node.rs"]
mod linked_list_node;

#[derive(Debug)]
pub struct LinkedList<T>
where
    T: PartialEq,
{
    head: Link<T>,
    tail: Link<T>,
    compare: Comparator<T>,
}

impl<T> LinkedList<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn new(compare_function: Option<Box<dyn Fn(&T, &T) -> i32>>) -> Self {
        LinkedList {
            head: None,
            tail: None,
            compare: Comparator::new(compare_function),
        }
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = LinkedListNode::new(value);
        new_node.borrow_mut().next = self.head.clone();
        self.head = Some(new_node.clone());
        if self.tail.is_none() {
            self.tail = Some(new_node);
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = LinkedListNode::new(value);
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.prepend(value);
        } else {
            let mut current = self.head.clone();
            let mut count = 0;
            while let Some(node) = current {
                if count == index - 1 {
                    let new_node = LinkedListNode::new(value);
                    new_node.borrow_mut().next = node.borrow_mut().next.take();
                    node.borrow_mut().next = Some(new_node);
                    if new_node.borrow().next.is_none() {
                        self.tail = Some(new_node);
                    }
                    break;
                }
                current = node.borrow().next.clone();
                count += 1;
            }
        }
    }

    pub fn delete(&mut self, value: T) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        let mut deleted_node = None;

        while let Some(head) = self.head.clone() {
            if self.compare.equal(&head.borrow().value, &value) {
                deleted_node = self.head.clone();
                self.head = head.borrow_mut().next.take();
            } else {
                break;
            }
        }

        let mut current = self.head.clone();
        while let Some(node) = current {
            if let Some(next) = node.borrow().next.clone() {
                if self.compare.equal(&next.borrow().value, &value) {
                    node.borrow_mut().next = next.borrow_mut().next.take();
                    deleted_node = Some(next);
                } else {
                    current = Some(next);
                }
            } else {
                break;
            }
        }

        if let Some(tail) = self.tail.clone() {
            if self.compare.equal(&tail.borrow().value, &value) {
                self.tail = current;
            }
        }

        deleted_node
    }

    pub fn find(
        &self,
        value: Option<&T>,
        callback: Option<&dyn Fn(&T) -> bool>,
    ) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if let Some(cb) = callback {
                if cb(&node.borrow().value) {
                    return Some(node);
                }
            } else if let Some(v) = value {
                if self.compare.equal(&node.borrow().value, v) {
                    return Some(node);
                }
            }
            current = node.borrow().next.clone();
        }
        None
    }

    pub fn delete_tail(&mut self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        if self.head.is_none() {
            return None;
        }

        let deleted_tail = self.tail.clone();

        if Rc::ptr_eq(self.head.as_ref().unwrap(), self.tail.as_ref().unwrap()) {
            self.head = None;
            self.tail = None;
        } else {
            let mut current = self.head.clone();
            while let Some(node) = current {
                if let Some(next) = node.borrow().next.clone() {
                    if Rc::ptr_eq(&next, self.tail.as_ref().unwrap()) {
                        node.borrow_mut().next = None;
                        self.tail = Some(node);
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
        let mut nodes = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            nodes.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }
        nodes
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.clone();
        self.tail = self.head.clone();
        while let Some(node) = current {
            let next = node.borrow_mut().next.take();
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
    fn test_linked_list() {
        let mut list = LinkedList::new(None);
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.to_vec(), vec![1, 2, 3]);

        list.prepend(0);
        assert_eq!(list.to_vec(), vec![0, 1, 2, 3]);

        list.delete(2);
        assert_eq!(list.to_vec(), vec![0, 1, 3]);

        list.reverse();
        assert_eq!(list.to_vec(), vec![3, 1, 0]);

        list.insert(2, 1);
        assert_eq!(list.to_vec(), vec![3, 2, 1, 0]);

        assert_eq!(list.delete_head().unwrap().borrow().value, 3);
        assert_eq!(list.delete_tail().unwrap().borrow().value, 0);
        assert_eq!(list.to_vec(), vec![2, 1]);
    }
}

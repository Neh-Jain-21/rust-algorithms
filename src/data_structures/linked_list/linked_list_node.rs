use std::{cell::RefCell, rc::Rc};

pub type Link<T> = Option<Rc<RefCell<LinkedListNode<T>>>>;

#[derive(Clone, Debug)]
pub struct LinkedListNode<T>
where
    T: Clone,
{
    pub value: T,
    pub next: Link<T>,
}

impl<T> LinkedListNode<T>
where
    T: Clone,
{
    pub fn new(value: T, next: Link<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LinkedListNode { value, next }))
    }
}

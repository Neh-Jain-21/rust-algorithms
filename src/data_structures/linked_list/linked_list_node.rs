pub type Link<T> = Option<Box<LinkedListNode<T>>>;

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
    pub fn new(value: T, next: Link<T>) -> Box<Self> {
        Box::new(LinkedListNode { value, next })
    }
}

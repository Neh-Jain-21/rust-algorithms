pub type Link<T> = Option<Box<LinkedListNode<T>>>;

pub struct LinkedListNode<T> {
    value: T,
    next: Link<T>,
}

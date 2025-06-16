struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            next: None,
        }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    length: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(value: T) -> Self {
        let mut node = Box::new(Node::new(value));
        let ptr: *mut Node<T> = &mut *node;
        SinglyLinkedList {
            head: Some(node),
            tail: ptr,
            length: 1,
        }
    }
}

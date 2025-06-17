use std::fmt::Debug;

use super::super::array::array::Array;
use super::super::stack::stack::Stack;

pub struct QueueOnStacks<T: Debug> {
    enqueue_stack: Stack<T>,
    dequeue_stack: Stack<T>,
}

impl<T: Debug> QueueOnStacks<T> {
    pub fn new() -> Self {
        QueueOnStacks {
            enqueue_stack: Stack::<T>::new(),
            dequeue_stack: Stack::<T>::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.enqueue_stack.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        // if self.dequeue_stack // TODO: Add len() to stack implementation
        if self.dequeue_stack.len() == 0 {}
        self.dequeue_stack.pop()
    }
}

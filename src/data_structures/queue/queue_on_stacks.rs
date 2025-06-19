use std::collections::btree_map::Range;
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
        if self.is_empty() {
            return None;
        }
        if self.dequeue_stack.is_empty() {
            self.move_items();
        }
        self.dequeue_stack.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        if self.dequeue_stack.is_empty() {
            self.move_items();
        }

        self.dequeue_stack.peek()
    }

    fn move_items(&mut self) {
        while let Some(value) = self.enqueue_stack.pop() {
            self.dequeue_stack.push(value);
        }
    }

    fn is_empty(&self) -> bool {
        self.enqueue_stack.len() + self.dequeue_stack.len() == 0
    }
}

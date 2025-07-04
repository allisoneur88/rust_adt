#![allow(dead_code)]

use std::fmt::Debug;

use super::super::stack::stack::Stack;

#[derive(Debug)]
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

    pub fn len(&self) -> usize {
        self.enqueue_stack.len() + self.dequeue_stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.enqueue_stack.len() + self.dequeue_stack.len() == 0
    }

    fn move_items(&mut self) {
        while let Some(value) = self.enqueue_stack.pop() {
            self.dequeue_stack.push(value);
        }
    }
}

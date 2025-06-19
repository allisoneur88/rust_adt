#![allow(dead_code)]

use std::fmt::Debug;

use super::super::array::array::Array;

#[derive(Debug)]
pub struct Stack<T: Debug> {
    data: Array<T>,
}

impl<T: Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Array::<T>::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get_by_index(self.data.len() - 1)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

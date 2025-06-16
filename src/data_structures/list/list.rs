use crate::List::*;

use std::fmt::Display;

pub enum List<T: Display> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: Display> List<T> {
    pub fn new() -> List<T> {
        Self::Nil
    }

    pub fn prepend(self, item: T) -> List<T> {
        Cons(item, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Cons(ref head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

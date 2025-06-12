#[derive(Debug)]
pub struct Array<T: std::fmt::Debug> {
    data: Vec<T>,
    capacity: usize,
}

impl<T: std::fmt::Debug> Array<T> {
    pub fn new() -> Self {
        Array {
            data: Vec::with_capacity(4),
            capacity: 4,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Array {
            data: Vec::with_capacity(capacity),
            capacity: capacity,
        }
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_all_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        if !self.is_out_of_range(index) {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut T> {
        if !self.is_out_of_range(index) {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) {
        if self.needs_resizing() {
            self.resize();
        }

        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            match self.data.pop() {
                Some(value) => Some(value),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn delete_at(&mut self, index: usize) {
        if self.is_out_of_range(index) {
            println!("Index out of range");
            return;
        }

        self.data.remove(index);
    }

    pub fn insert_at(&mut self, index: usize, item: T) {
        if self.is_out_of_range(index) {
            println!("Index out of range");
            return;
        }

        if self.needs_resizing() {
            self.resize();
        }

        self.data.insert(index, item);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn print(&self) {
        println!("{:#?}", self)
    }

    fn resize(&mut self) {
        self.capacity *= 2;
        let mut new_data: Vec<T> = Vec::with_capacity(self.capacity);

        new_data.extend(self.data.drain(..));

        self.data = new_data;
    }

    fn needs_resizing(&self) -> bool {
        self.data.len() == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn is_out_of_range(&self, index: usize) -> bool {
        index >= self.data.len()
    }
}

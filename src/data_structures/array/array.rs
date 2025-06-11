#[derive(Debug)]
pub struct Array<T: std::fmt::Debug> {
    data: Vec<T>,
    count: usize,
}

impl<T: std::fmt::Debug> Array<T> {
    pub fn new() -> Self {
        Array {
            data: Vec::new(),
            count: 0,
        }
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        if self.count > index {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count != 0 {
            self.count -= 1;
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

        self.count -= 1;
    }

    pub fn insert_at(&mut self, index: usize, item: T) {
        if self.is_out_of_range(index) {
            println!("Index out of range");
            return;
        }

        self.data.insert(index, item);

        self.count += 1;
    }

    pub fn print(&self) {
        println!("{:#?}", self)
    }

    fn is_out_of_range(&self, index: usize) -> bool {
        index >= self.count
    }
}

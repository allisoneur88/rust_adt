const ARRAY_SIZE: usize = 8;

#[derive(Debug)]
pub struct Array {
    data: [isize; ARRAY_SIZE],
    count: usize,
}

impl Array {
    pub fn new() -> Self {
        Array {
            data: [0; ARRAY_SIZE],
            count: 0,
        }
    }

    pub fn get_all(&self) -> &[isize] {
        &self.data
    }

    pub fn get_by_index(&self, index: usize) -> Option<&isize> {
        if self.count > index {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn push(&mut self, item: isize) {
        self.data[self.count] = item;
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<isize> {
        if self.count != 0 {
            self.count -= 1;
            self.data[self.count] = 0;
            Some(self.data[self.count])
        } else {
            self.count -= 1;
            None
        }
    }

    pub fn delete_at(&mut self, index: usize) {
        if self.is_out_of_range(index) {
            println!("Index out of range");
            return;
        }

        self.shift_left(index);

        self.count -= 1;

        self.data[self.count] = 0;
    }

    pub fn insert_at(&mut self, index: usize, item: isize) {
        if self.is_out_of_range(index) {
            println!("Index out of range");
            return;
        }

        self.shift_right(index);

        self.data[index] = item;

        self.count += 1;
    }

    pub fn print(&self) {
        println!("{:#?}", self)
    }

    fn is_out_of_range(&self, index: usize) -> bool {
        index >= self.count
    }

    fn is_overflow(&self) -> bool {
        self.count == ARRAY_SIZE
    }

    fn shift_left(&mut self, index: usize) {
        let mut i = index;

        while i < self.count - 1 {
            self.data[i] = self.data[i + 1];
            i += 1;
        }
    }

    fn shift_right(&mut self, index: usize) {
        let mut i = self.count;

        while i > index {
            self.data[i] = self.data[i - 1];
            i -= 1;
        }
    }
}

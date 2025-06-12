use super::super::array::array::Array;

use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::Error;
use std::io::ErrorKind::NotFound;

#[derive(Debug)]
struct Pair<K: Debug + Hash + Eq + Clone, V: Debug + Clone> {
    key: K,
    value: V,
}

impl<K: Debug + Hash + Eq + Clone, V: Debug + Clone> Pair<K, V> {
    fn new(key: K, value: V) -> Self {
        Pair {
            key: key,
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Hashtable<K: Debug + Hash + Eq + Clone, V: Debug + Clone> {
    data: Array<Array<Pair<K, V>>>,
    keys: Array<K>,
    capacity: usize,
    resize_threshhold: f64,
}

impl<K: Debug + Hash + Eq + Clone, V: Debug + Clone> Hashtable<K, V> {
    pub fn new() -> Self {
        let capacity = 50;

        let mut data: Array<Array<Pair<K, V>>> = Array::with_capacity(capacity);

        for _ in 0..capacity {
            data.push(Array::new()); //  data.push(Array::<Pair<K, V>>::new());
        }

        Hashtable {
            data: data,
            keys: Array::new(),
            capacity: capacity,
            resize_threshhold: 0.75,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut data: Array<Array<Pair<K, V>>> = Array::with_capacity(capacity);

        for _ in 0..capacity {
            data.push(Array::new()); //  data.push(Array::<Pair<K, V>>::new());
        }

        Hashtable {
            data: data,
            keys: Array::new(),
            capacity: capacity,
            resize_threshhold: 0.75,
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        let index = self.simple_hash(&key, self.capacity);

        match self.data.get_by_index_mut(index) {
            Some(bucket) => {
                let mut value = Some(value);
                let mut found = false;

                for pair in bucket.get_all_mut() {
                    if pair.key == key {
                        pair.value = value.take().unwrap();
                        found = true;
                        break;
                    }
                }

                if !found {
                    bucket.push(Pair::new(key.clone(), value.take().unwrap()));

                    if !self.keys.get_all().contains(&key) {
                        self.keys.push(key.clone())
                    }
                }

                if self.needs_resizing() {
                    self.resize_and_rehash();
                }
            }

            None => {
                eprintln!("Error: Bucket not found at index {}", index);
            }
        }
    }

    pub fn get(&self, key: &K) -> Result<&V, Error> {
        let index = self.simple_hash(key, self.capacity);

        match self.data.get_by_index(index) {
            Some(bucket) => {
                for pair in bucket.get_all() {
                    if &pair.key == key {
                        return Ok(&pair.value);
                    }
                }
                Err(Error::new(NotFound, "Key not found"))
            }
            None => Err(Error::new(NotFound, "Key not found")),
        }
    }

    fn resize_and_rehash(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_data: Array<Array<Pair<K, V>>> = Array::with_capacity(new_capacity);

        for _ in 0..new_capacity {
            new_data.push(Array::new()); //  data.push(Array::<Pair<K, V>>::new());
        }

        for key in self.keys.get_all() {
            if let Ok(value_ref) = self.get(key) {
                let item = Pair::new(key.clone(), value_ref.clone());
                let new_index = self.simple_hash(key, new_capacity);
                let new_bucket = new_data.get_by_index_mut(new_index).unwrap();

                new_bucket.push(item);
            } else {
                eprintln!("Warning: key {:?} was in keys but not found in table", key);
            }
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }

    fn needs_resizing(&self) -> bool {
        self.keys.len() as f64 / self.capacity as f64 > self.resize_threshhold
    }

    fn simple_hash(&self, key: &K, capacity: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash as usize) % capacity
    }
}

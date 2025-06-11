use super::super::array::array::Array;

#[derive(Debug)]
struct Pair<K: std::fmt::Debug, V: std::fmt::Debug> {
    key: K,
    value: V,
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> Pair<K, V> {
    fn new(key: K, value: V) -> Self {
        Pair {
            key: key,
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Hashtable<K: std::fmt::Debug, V: std::fmt::Debug> {
    data: Array<Array<Pair<K, V>>>,
    keys: Array<K>,
    size: usize,
    resize_threshhold: f64,
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> Hashtable<K, V> {
    fn new(size: usize) -> Self {
        let mut data: Array<Array<Pair<K, V>>> = Array::new();

        for _ in 0..size {
            data.push(Array::new());
        }

        Hashtable {
            data: data,
            keys: Array::new(),
            size: size,
            resize_threshhold: 0.75,
        }
    }
}

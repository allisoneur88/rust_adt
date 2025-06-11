mod data_structures;
use data_structures::array::array::Array;
use data_structures::hashtable::hashtable::Hashtable;

fn main() {
    test_array();
}

fn test_array() {
    let mut arr: Array<isize> = Array::new();

    let mut i = 0;

    while i < 7 {
        arr.push(i);
        i += 1;
    }

    arr.print();

    arr.insert_at(1, 69);

    arr.print();

    arr.delete_at(5);

    arr.print();
}

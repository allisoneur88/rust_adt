mod data_structures;
use std::ptr::hash;

use data_structures::array::array::Array;
use data_structures::hashtable::hashtable::Hashtable;
use data_structures::list::list::List;
use data_structures::singly_linked_list::singly_linked_list::SinglyLinkedList;

fn main() {
    // test_hash();
    test_list();
}

fn test_hash() {
    let mut hash: Hashtable<String, usize> = Hashtable::with_capacity(1);

    let age = "age".to_string();

    hash.set(age.to_string(), 29);

    hash.set("height_cm".to_string(), 189);

    if let Ok(val) = hash.get(&age) {
        println!("age: {val}")
    }

    println!("{:#?}", hash);
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

    match arr.get_by_index(7) {
        Some(val) => println!("{}", val),
        None => println!("Nothing there"),
    }
}

fn test_list() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("length: {}", list.len());
    println!("{}", list.stringify());
}

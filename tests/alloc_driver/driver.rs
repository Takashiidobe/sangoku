#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

pub fn vector(mut values: Vec<u16>, value: u16) -> Vec<u16> {
    values.push(value);
    values.insert(0, value);
    values.sort_unstable();
    values.dedup();
    values
}

pub fn string(mut value: String) -> String {
    value.push_str("abc");
    value.push('é');
    value.replace('a', "b")
}

pub fn boxed(value: u16) -> Box<u16> {
    Box::new(value)
}

pub fn shared(value: u16) -> Rc<u16> {
    let value = Rc::new(value);
    value.clone()
}

pub fn map(mut values: BTreeMap<u16, u16>, key: u16) -> Option<u16> {
    values.insert(key, key);
    values.remove(&key)
}

pub fn set(mut values: BTreeSet<u16>, key: u16) -> bool {
    values.insert(key);
    values.remove(&key)
}

pub fn queue(mut values: VecDeque<u16>, value: u16) -> Option<u16> {
    values.push_back(value);
    values.pop_front()
}

pub fn heap(mut values: BinaryHeap<u16>, value: u16) -> Option<u16> {
    values.push(value);
    values.pop()
}

pub fn list(mut values: LinkedList<u16>, value: u16) -> Option<u16> {
    values.push_back(value);
    values.pop_front()
}

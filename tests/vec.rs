use std::cell::Cell;

use sangoku_vec::FixedVec;

#[test]
fn push_fills_to_capacity_then_returns_the_value() {
    let mut vec = FixedVec::<u8, 2>::new();
    assert_eq!(vec.push(1), Ok(()));
    assert_eq!(vec.push(2), Ok(()));
    assert_eq!(vec.push(3), Err(3));
    assert_eq!(vec.as_slice(), &[1, 2]);
    assert_eq!((vec.len(), vec.capacity(), vec.is_empty()), (2, 2, false));
}

#[test]
fn pop_returns_values_in_reverse_order() {
    let mut vec = FixedVec::<u8, 4>::new();
    vec.push(1).unwrap();
    vec.push(2).unwrap();
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
    assert!(vec.is_empty());
}

#[test]
fn slice_access_and_iteration_cover_only_initialized_elements() {
    let mut vec = FixedVec::<u8, 8>::new();
    vec.push(10).unwrap();
    vec.push(20).unwrap();
    vec[0] += 1;
    assert_eq!(vec.iter().copied().sum::<u8>(), 31);
    for value in &mut vec {
        *value += 1;
    }
    assert_eq!((&vec).into_iter().copied().collect::<Vec<_>>(), [12, 21]);
}

struct Counted<'a>(&'a Cell<usize>);

impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn drop_and_clear_drop_each_initialized_element_once() {
    let drops = Cell::new(0);
    let mut vec = FixedVec::<Counted, 4>::new();
    for _ in 0..3 {
        assert!(vec.push(Counted(&drops)).is_ok());
    }
    drop(vec.pop());
    assert_eq!(drops.get(), 1);
    vec.clear();
    assert_eq!(drops.get(), 3);
    assert!(vec.push(Counted(&drops)).is_ok());
    drop(vec);
    assert_eq!(drops.get(), 4);
}

#[test]
fn rejected_push_hands_the_value_back_without_dropping_it() {
    let drops = Cell::new(0);
    let mut vec = FixedVec::<Counted, 1>::new();
    assert!(vec.push(Counted(&drops)).is_ok());
    let rejected = vec.push(Counted(&drops)).unwrap_err();
    assert_eq!(drops.get(), 0);
    drop(rejected);
    assert_eq!(drops.get(), 1);
}

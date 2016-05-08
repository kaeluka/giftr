extern crate giftr;

use giftr::refs::*;
use giftr::refs::functional::Ref as Ref;
//use giftr::refs::imperative::Ref as Ref;

#[derive(Clone)]
struct Node<T: Clone> {
    next: Ref<Option<Node<T>>>,
    val:T,
}

impl <T: Clone> Node<T> {
    fn len(&self) -> i32 {
        match *(self.next) {
            Some(ref node) => 1 + node.len(),
            None           => 1
        }
    }
}

#[derive(Clone)]
struct List<T: Clone> {
    first : Ref<Option<Node<T>>>,
}

impl <T: Clone> List<T> {
    fn new() -> List<T> {
        List { first: Ref::new(None) }
    }

    fn add(&mut self, x:T) {
        let newFirst = Node { val: x, next: _move(&mut self.first) };
        self.first = Ref::new(Some(newFirst));
    }

    fn len(&self) -> i32 {
        match *(self.first) {
            Some(ref node) => node.len(),
            None => 0
        }
    }
}

#[test]
fn lst_len() {
    println!("=== LST_LEN ==============");
    let mut lst = Ref::new(List::new());
    assert!(0 == lst.len());

    lst.add(1);
    assert!(1 == lst.len());

    lst.add(2);
    assert!(2 == lst.len());

    lst.add(3);
    assert!(3 == lst.len());
}

#[test]
fn lst_copy() {
    println!("=== LST_COPY ==============");
    let mut lst1 : Ref<List<i32>> = Ref::new(List::new());
    lst1.add(1);

    let mut lst2 : Ref<List<i32>> = Ref::null();
    lst2.cp(&lst1);

    lst1.add(2);
    lst1.add(3);

    assert!(3 == lst1.len());
    assert!(1 == lst2.len());
}


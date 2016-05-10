extern crate giftr;

use giftr::refs::*;
use giftr::refs::functional::Ref as Ref;
//use giftr::refs::imperative::Ref as Ref;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
struct Node<T: Clone+Debug> {
    next: Ref<Option<LoudClone<Node<T>>>>,
    elt:LoudClone<T>,
}

impl <T: Clone+Debug> Node<T> {
    fn len(&self) -> i32 {
        match *(self.next) {
            Some(ref node) => 1 + node.len(),
            None           => 1
        }
    }
}

#[derive(Debug)]
struct LoudClone<T: Debug> {
    val: T,
}

impl <T: Debug> LoudClone<T> {
    fn new(x:T) -> Self {
        LoudClone { val: x }
    }
}

impl <T: Clone + Debug> Clone for LoudClone<T> {
    fn clone(&self) -> Self {
        println!("cloning <{:?}>", self.val);
        let ret = LoudClone { val: self.val.clone() };
        println!("cloned  <{:?}>", self.val);
        ret
    }
}

impl <T: Debug> Deref for LoudClone<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl <T: Debug> DerefMut for LoudClone<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

#[derive(Clone, Debug)]
struct List<T: Clone+Debug> {
    first : Ref<Option<LoudClone<Node<T>>>>,
}

impl <T: Clone+Debug> List<T> {
    fn new() -> List<T> {
        List { first: Ref::new(None) }
    }

    fn add(&mut self, x:T) {
        let newFirst = Node { elt: LoudClone::new(x), next: _move(&mut self.first) };
        self.first = Ref::new(Some(LoudClone::new(newFirst)));
    }

    fn replace_first(&mut self, x:T) {
        match *(self.first) {
            Some(ref mut node) => *node.elt = x,
            None => panic!("must have first")
        }
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
    let mut lst1 = Ref::new(List::new());
    lst1.add(1);
    let mut lst2 : Ref<List<i32>> = Ref::null();
    lst1.add(2);

    lst2 = lst1.clone();

    lst1.add(3);

    assert!(3 == lst1.len());
    assert!(2 == lst2.len());
}

#[test]
fn lst_replace_first() {
    println!("=== LST_REPLACE_FIRST ==============");
    let mut lst = Ref::new(List::new());
    lst.add(3);
    lst.add(2);
    lst.add(-1);
    lst.replace_first(1);
}

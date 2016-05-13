extern crate giftr;

use giftr::refs::*;

//// Based on which of the two lines you include, the
//// counter will either be immutable or imperative:

//use giftr::refs::functional::Ref as Ref;
use giftr::refs::imperative::Ref as Ref;

//// The two references have two major differences:
////  - for the functional one, cloning is essentially a no-op, but
////    dereferencing the pointer for writing makes a shallow clone
////    of the object.
////  - for the imperative one, cloning is an actual deep clone, but
////    there is no extra cost for mutations.

//// The two implementations provide the same interface -- by just
//// programming to that interface, you can write code that is
//// polymorphic with regards to mutability.

macro_rules! uq {
    ($t:ty) => (FRef<$t>)
}

macro_rules! new_uq {
    ($e:expr) => (FRef::new($e))
}

#[derive(Clone,Debug)]
pub struct Counter {
    c: Ref<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter { c: Ref::new(0) }
    }

    fn inc(&mut self) {
        *self.c = *self.c+1;
    }

    fn get(&self) -> i32 {
        *self.c
    }
}

#[test]
fn counter() {
    //// This test will pass for both implementations
    let mut c1 = Ref::new(Counter::new());
    c1.inc();
    let c2 = c1.clone(); //// here we make a semantic copy.
                         //// the effects on c1 will be disjoint
                         //// from the effects on c2.
                         //// The two reference implementations
                         //// achieve this by different means though.
                         //// The imperative one will make a deep clone here,
                         //// the functional one will not.

    //// here, we change an object. The imperative version has no overhead,
    //// the functional one will make a shallow clone.
    c1.inc();
    assert_eq!(2, c1.get());
    assert_eq!(1, c2.get());
}

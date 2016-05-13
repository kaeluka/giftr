extern crate giftr;

use giftr::refs::*;

//use::giftr::refs::imperative::Ref as Ref;
use::giftr::refs::functional::Ref as Ref;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Aliased<T: Clone> {
    x : Ref<Rc<RefCell<T>>>,
}

impl <T: Clone> Aliased<T> {
    fn new(x: T) -> Aliased<T> {
        Aliased { x : Ref::new(Rc::new(RefCell::new(x))) }
    }
}

#[test]
fn aliasing_test() {
    let mut a : Ref<Aliased<i8>> = Ref::new(Aliased::new(0));
    let b = a.clone();
    *a.x.borrow_mut() += 1;
    assert!(*a.x.borrow() == 1);
    assert!(*b.x.borrow() == 1);
}

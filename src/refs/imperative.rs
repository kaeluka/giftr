use refs::GiftRef;
use refs::GiftMutRef;
use std::ops::{Deref, DerefMut};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ref<T> {
    _ptr : Option<Rc<RefCell<T>>>,
}

impl <T> Ref<T> {

    pub fn rd_mut(&mut self) -> &mut T {
        match self._ptr {
            Some(ref r) => {
                let p : *mut T = &mut *r.borrow_mut();
                unsafe { &mut *p }
            }
            None => {
                panic!("null pointer dereference")
            }
        }
    }
}

impl <T: Clone> Clone for Ref<T> {
    fn clone(&self) -> Self {
        let mut ret : Ref<T> = Ref::null();
        ret.cp(self);
        ret
    }
}

impl <T> GiftRef<T> for Ref<T> where T: Clone {

    type Mut = Ref<T>;

    fn null() -> Self {
        Ref { _ptr: None }
    }

    fn new(t: T) -> Self {
        Ref { _ptr: Some(Rc::new(RefCell::new(t))) }
    }

    fn cp(&mut self, source : &Self) {
        match source._ptr {
            Some(ref r) => {
                self._ptr = Some(Rc::new(RefCell::new(r.borrow().clone())))
            }
            None => {
                self._ptr = None
            }
        }
    }

    fn mutable(&mut self) -> Self::Mut {
        Ref { _ptr: self._ptr.clone() }
    }

    fn rd(&self) -> &T {
        match self._ptr {
            Some(ref r) => {
                let p : *const T = &*r.borrow_mut();
                unsafe { &*p }
            }
            None    => panic!("reading null pointer")
        }
    }
}

impl <T:Clone> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.rd()
    }
}

impl <'a, T> GiftMutRef<T> for Ref<T> {
    fn rd(&mut self) -> &mut T {
        match self._ptr {
            Some(ref r) => {
                let p : *mut T = &mut *r.borrow_mut();
                unsafe { &mut *p }
            }
            None    => panic!("reading null pointer")
        }
    }
}

impl <T: Clone> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rd_mut()
    }
}

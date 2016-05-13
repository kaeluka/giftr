extern crate refcownt;

use refs::GiftRef;
use refs::GiftMutRef;
use std::ops::{Deref,DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Ref<T: Clone> {
    _ptr : Rc<T>,
}

impl <'c, T: Clone> GiftRef<T> for Ref<T> {

    type Mut = Ref<T>;

    #[inline]
    fn new(t:T) -> Self {
        Ref { _ptr: Rc::new(t) }
    }

    #[inline]
    fn cp(&mut self, source : &Self) {
        //copying is just creating an alias:
        self._ptr = source._ptr.clone();
    }


    fn rd<'a>(&'a self) -> &'a T {
        &*self._ptr
    }

    fn into_inner(self) -> T {
        match Rc::try_unwrap(self._ptr) {
            Ok(x) => x,
            Err(_ptr) => (*_ptr).clone()
        }
    }
}

impl <'c, T: Clone> Clone for Ref<T> {
    #[inline]
    fn clone(&self) -> Self {
        Ref { _ptr: self._ptr.clone() }
    }
}

impl <'c, T: Clone> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.rd()
    }
}

impl <'c, T: Clone> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut T {
        // We clone the element when dereferencing mutably. We also update the
        // current reference (self) to refer to the new version in order for the
        // variable to reflect the changes.
//        self._ptr = self._ptr.clone();
        Rc::make_mut(&mut self._ptr)
    }
}

impl <'c, T: Clone> GiftMutRef<T> for Ref<T> {
    fn rd(&mut self) -> &mut T {
        self.deref_mut()
    }
}

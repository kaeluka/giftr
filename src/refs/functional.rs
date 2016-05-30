use refs::GiftRef;
use std::ops::{Deref,DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Ref<T> {
    pub _ptr : Rc<T>, //public for the purpose of implementing `Drop`
}

impl <T> Ref<T> {
    fn rd<'a>(&'a self) -> &'a T {
        &*self._ptr
    }
}

impl <'c, T: Clone> GiftRef<T> for Ref<T> {

    #[inline]
    fn new(t:T) -> Self {
        Ref { _ptr: Rc::new(t) }
    }

    fn apply<F: FnOnce(T) -> T>(&mut self, f: F)
        where Self: Sized
    {
        let x = self.clone().consume();
        self._ptr = Rc::new(f(x));
    }

    fn consume(self) -> T {
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

impl <'c, T> Deref for Ref<T> {
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
        Rc::make_mut(&mut self._ptr)
    }
}

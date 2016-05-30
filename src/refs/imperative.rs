use refs::GiftRef;
//use refs::GiftMutRef;
use std::ops::{Deref, DerefMut};
use std::boxed::Box;
use std::ptr::drop_in_place;

#[derive(Debug)]
pub struct Ref<T> {
    pub _ptr : *mut T, //Public for the purpose of implementing `Drop`
}

impl <T> Ref<T> {

    pub fn rd(&self) -> &T {
        unsafe { &*self._ptr }
    }

    pub fn rd_mut(&self) -> &mut T {
        unsafe { &mut *self._ptr }
    }
}

impl <T: Clone> Clone for Ref<T> {
    #[inline]
    fn clone(&self) -> Self {
        let t : T = self.rd().clone();
        Ref { _ptr: Box::into_raw(Box::new(t)) }
    }
}

impl <T> GiftRef<T> for Ref<T> where T: Clone {

    #[inline]
    fn new(t: T) -> Self {
        Ref { _ptr: Box::into_raw(Box::new(t)) }
    }

    fn apply<F: FnOnce(T) -> T>(&mut self, f: F)
        where Self: Sized
    {
        let b = unsafe { Box::from_raw(self._ptr) };
        self._ptr = Box::into_raw(Box::new( f(*b) ))

    }

    fn consume(self) -> T {
        let Ref { _ptr: ptr } = self;
        *unsafe { Box::from_raw(ptr) }
    }

}

impl <T> Drop for Ref<T> {
    fn drop(&mut self) {
        unsafe { drop_in_place(self._ptr) };
    }
}

impl <T:Clone> Deref for Ref<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.rd()
    }
}

impl <T: Clone> DerefMut for Ref<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rd_mut()
    }
}

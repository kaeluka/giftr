use refs::GiftRef;
use refs::GiftMutRef;
use std::ops::{Deref, DerefMut};
use std::boxed::Box;
use std::ptr::drop_in_place;

#[derive(Debug)]
pub struct Ref<T> {
    _ptr : *mut T,
}

impl <T> Ref<T> {

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

    type Mut = Ref<T>;

    #[inline]
    fn new(t: T) -> Self {
        Ref { _ptr: Box::into_raw(Box::new(t)) }
    }

    fn cp(&mut self, source : &Self) {
        let cln : T = source.rd().clone();
        self._ptr = Box::into_raw(Box::new(cln));
    }

    fn rd(&self) -> &T {
        self.rd_mut()
    }

    fn into_inner(self) -> T {
        let Ref { _ptr: ptr } = self;
        *unsafe { Box::from_raw(ptr) }
    }

}

impl <T> Drop for Ref<T> {
    fn drop(&mut self) {
        println!("dropping ptr: {:?}", self._ptr);
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

impl <'a, T: Clone> GiftMutRef<T> for Ref<T> {
    fn rd(&mut self) -> &mut T {
        self.rd_mut()
    }
}

impl <T: Clone> DerefMut for Ref<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rd_mut()
    }
}

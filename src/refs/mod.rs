use std::ops::{Deref, DerefMut};
use std::mem::replace;

mod mem;

pub trait GiftRef<T: Clone> : Deref<Target=T> + DerefMut + Clone {
    type Mut : GiftMutRef<T>;
    fn null() -> Self;
    fn new(T) -> Self;
    fn cp(&mut self, &Self);
    fn mutable(&mut self) -> Self::Mut;
    fn rd(&self) -> &T;
}

pub trait GiftMutRef<T> {
    fn rd(&mut self) -> &mut T;
}

pub fn _replace<T: Clone, R : GiftRef<T>>(r: &mut R, x:T) -> R {
    replace(r, R::new(x))
}

pub fn _move<T: Clone, R : GiftRef<Option<T>>>(r: &mut R) -> R {
    _replace(r, None)
}

pub fn _copy<T: Clone, R : GiftRef<Option<T>>>(r: &mut R) -> R {
    panic!("not implemented")
}

pub mod imperative;

pub mod functional;

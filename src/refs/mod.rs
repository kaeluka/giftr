use std::ops::{Deref, DerefMut};
use std::mem::replace;

mod mem;

pub trait GiftRef<T: Clone> : Deref<Target=T> + DerefMut + Clone {
    type Mut : GiftMutRef<T>;
    fn null() -> Self;
    fn new(T) -> Self;
    fn cp(&mut self, &Self);
    fn alias<'a,'b>(&mut self, &'a mut Self, &'b mut Self);
    fn mutable(&mut self) -> Self::Mut;
    fn rd(&self) -> &T;
}

pub trait GiftMutRef<T> {
    fn rd(&mut self) -> &mut T;
}

#[inline]
pub fn _replace<T: Clone, R : GiftRef<T>>(r: &mut R, x:T) -> R {
    replace(r, R::new(x))
}

#[inline]
pub fn _move<T: Default+Clone, R : GiftRef<T>>(r: &mut R) -> R {
    _replace(r, Default::default())
}

#[inline]
pub fn _copy<T: Default+Clone, R : GiftRef<T>>(r: &R) -> R {
    r.clone()
}

pub mod imperative;

pub mod functional;

mod imp_tests {
    use refs::GiftRef;
    use refs::imperative::Ref;

    fn print_x(x: i32) {
        println!("x={}", x)
    }

    #[test]
    pub fn rd() {
        println!("==============================");
        let r = Ref::new(12);
        print_x(*r);
        assert!(*r == 12);
    }

    #[test]
    pub fn mutate() {
        let mut x = Ref::new(12);
        print_x(*x);
        *x += 1;
        assert!(*x == 13);
    }

    #[test]
    pub fn copy() {
        let mut r1 : Ref<i32> = Ref::new(12);
        let     r2 : Ref<i32> = Ref::new(24);
        assert!(*r1 == 12);
        assert!(*r2 == 24);

        r1.cp(&r2);

        assert!(*r1 == 24);
        assert!(*r2 == 24);

        *r1+=1;

        println!("r1={}", *r1);
        println!("r2={}", *r2);
        assert!(*r1 == 25);
        assert!(*r2 == 24); //
    }
}

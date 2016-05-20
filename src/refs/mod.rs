use std::ops::{Deref, DerefMut};
use std::mem::replace;
use std::cell::RefCell;

mod mem;

pub trait GiftRef<T> : Deref<Target=T>+DerefMut {
    fn new(T) -> Self where Self: Sized;
    fn into_inner(self) -> T;

    fn to_dyn(self) -> DynGiftRef<T>
        where Self: Sized + 'static
    {
        DynGiftRef { _ptr: Box::new(RefCell::new(self)) as Box<RefCell<GiftRef<T, Target=T>>> }
    }
}

pub struct DynGiftRef<T> {
    _ptr: Box<RefCell<GiftRef<T, Target=T>>>,
}

impl <T: Clone + 'static> Clone for DynGiftRef<T> {
    fn clone(&self) -> Self {
        use refs::functional::Ref;
        let cln : T = (**self).clone();
        Ref::new(cln).to_dyn()
    }
}

impl <T> Deref for DynGiftRef<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &Self::Target {
        let p : *const T = &**self._ptr.borrow();
        unsafe { &*p }
    }
}

impl <T> DerefMut for DynGiftRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        let p : *mut T = &mut **self._ptr.borrow_mut();
        unsafe { &mut *p }
    }
}

#[inline]
pub fn _replace<T, R : GiftRef<T>>(r: &mut R, x:T) -> R {
    replace(r, R::new(x))
}

#[inline]
#[deprecated]
pub fn _move_opt<T>(o: &mut Option<T>) -> Option<T> {
    replace(o, None)
}

/// Take a reference and replace the source variable with an empty (default) value.
///
/// # Examples
///
/// ```
/// use giftr::refs::{GiftRef, _move};
/// use giftr::refs::imperative::Ref as Ref;
/// let mut r = Ref::new(Some(12i8));
/// let     s = _move(&mut r);
/// println!("r={:?}", *r); // prints "r=None"
/// ```
#[inline]
pub fn _move<T: Default, R : GiftRef<T>>(r: &mut R) -> R {
    _replace(r, Default::default())
}

pub mod imperative;

pub mod functional;

pub mod dummy;

#[cfg(test)]
mod imp_tests {
    use refs::GiftRef;
    use refs::imperative::Ref;

    #[cfg(test)]
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

        r1 = r2.clone();

        assert!(*r1 == 24);
        assert!(*r2 == 24);

        *r1+=1;

        println!("r1={}", *r1);
        println!("r2={}", *r2);
        assert!(*r1 == 25);
        assert!(*r2 == 24);
    }
}

#[cfg(test)]
mod fun_tests {
    use refs::GiftRef;
    use refs::functional::Ref;

    #[cfg(test)]
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
        assert_eq!(13, *x);
    }

    #[test]
    pub fn copy() {
        let mut r1 : Ref<i32> = Ref::new(12);
        let     r2 : Ref<i32> = Ref::new(24);
        assert!(*r1 == 12);
        assert!(*r2 == 24);

        r1 = r2.clone();

        assert!(*r1 == 24);
        assert!(*r2 == 24);

        *r1+=1;

        println!("r1={}", *r1);
        println!("r2={}", *r2);
        assert!(*r1 == 25);
        assert!(*r2 == 24);
    }
}

#[cfg(test)]
mod dyn_tests {

    use refs::GiftRef;
    use refs::DynGiftRef;
    use refs::functional::Ref as FRef;
    use refs::functional::Ref as IRef;

    #[derive(Clone)]
    struct Node(Option<DynGiftRef<Node>>);

    fn len(x: &Node) -> i32 {
        match x {
            &Node(Some(ref r)) => 1 + len(&*r),
            &Node(None)        => 1
        }
    }

    #[test]
    fn dynamic() {
        let n = Node(None);
        let n = Node(Some(FRef::new(n).to_dyn()));
        let n = Node(Some(IRef::new(n).to_dyn()));
        let n = Node(Some(FRef::new(n).to_dyn()));

        assert_eq!(4, len(&n))
    }
}

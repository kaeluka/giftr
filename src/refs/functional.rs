
use refs::GiftRef;
use refs::GiftMutRef;
use std::ops::{Deref,DerefMut};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ref<T> {
    _ptr : Option<Rc<RefCell<T>>>,
}

pub struct MutRef<T> {
    _ptr: Rc<RefCell<T>>,
}

impl <T: Clone> GiftRef<T> for Ref<T> {

    type Mut = MutRef<T>;

    #[inline]
    fn null() -> Self {
        Ref { _ptr: None }
    }

    #[inline]
    fn new(t:T) -> Self {
        Ref { _ptr: Some(Rc::new(RefCell::new(t))) }
    }

    #[inline]
    fn cp(&mut self, source : &Self) {
        //copying is just creating an alias:
        self._ptr = source._ptr.clone()
    }

    fn alias<'a,'b>(&mut self, _x: &'a mut Self, _y: &'b mut Self) {
        panic!("alias not impl!")
    }

    fn mutable<'a>(&'a mut self) -> Self::Mut {
        // We clone the element when dereferencing mutably. We also update the
        // current reference (self) to refer to the new version in order for the
        // variable to reflect the changes.
        let ret;
        match self._ptr {
            Some(ref mut r) => {
                let valref : &T = &*r.borrow();
                ret = MutRef {
                    _ptr : Rc::new(RefCell::new(valref.clone()))
                };
            }
            None => panic!("null pointer dereference"),
        };
        self._ptr = Some(ret._ptr.clone());
        ret
    }

    fn rd<'a>(&'a self) -> &'a T {
        match self._ptr {
            Some(ref r) => {
                let ptr : *const T = &*r.borrow();
                unsafe { &*ptr }
            }
            None        => panic!("null pointer dereference")
        }
    }

    fn into_inner(self) -> T {
        match self {
            Ref { _ptr: Some(rc) } => {
                match Rc::try_unwrap(rc) {
                    Ok(refcell) => refcell.into_inner(),
                    Err(rc)     => rc.borrow().clone(),
                }
            }
            Ref { _ptr: None} => panic!("bug"),
        }
    }
}

impl <T: Clone> Clone for Ref<T> {
    #[inline]
    fn clone(&self) -> Self {
        let mut ret : Ref<T> = Ref::null();
        ret.cp(self);
        ret
    }
}

impl <T: Clone> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.rd()
    }
}

impl <T: Clone> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ret : *mut T = self.mutable().rd();
        unsafe { &mut *ret }
    }
}

impl <T> GiftMutRef<T> for MutRef<T> {
    fn rd(&mut self) -> &mut T {
        let ptr : *mut T = &mut *self._ptr.borrow_mut();
        unsafe { &mut *ptr }
    }
}

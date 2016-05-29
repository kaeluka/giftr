use std::iter::Iterator;
use std::slice::{Iter, IterMut};
use std::default::Default;
use std::mem;
use std::vec::*;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result};

pub struct ISpine<T> {
    data : UnsafeCell<Vec<T>>,
}

impl <T: Debug> Debug for ISpine<T> {
    fn fmt(&self, fmtr : &mut Formatter) -> Result {
        self.data().fmt(fmtr)
    }
}

impl <T: Clone> Clone for ISpine<T> {
    fn clone(&self) -> ISpine<T> {
        let inner_cln : Vec<T>= unsafe { &*self.data.get() }.clone();
        ISpine { data: UnsafeCell::new(inner_cln) }
    }
}

impl <T> ISpine<T> {

    fn data(&self) -> &Vec<T> {
        unsafe { &*self.data.get() }
    }

    fn data_mut(&mut self) ->&mut  Vec<T> {
        unsafe { &mut *self.data.get() }
    }

    pub fn empty(&self) -> bool {
        self.data().len() == 0
    }

    pub fn add(&mut self, x : T) {
        self.data_mut().insert(0, x)
    }

    pub fn pop1(&mut self) -> Option<T> {
        if self.data().len() > 0 {
            Some(self.data_mut().remove(0))
        } else {
            None
        }
    }

    pub fn take(&mut self, n : usize) -> ISpine<T> {
        let second = UnsafeCell::new(self.data_mut().split_off(n));
        let tmp = mem::replace(&mut self.data, second);
        ISpine { data: tmp }
    }

    pub fn take_from(&mut self, n : usize) -> ISpine<T> {
        ISpine { data: UnsafeCell::new(self.data_mut().split_off(n)) }
    }

    pub fn at<'a>(&'a mut self) -> ISpineLocationIter<'a, T> {
        ISpineLocationIter { root : self,
                             idx  : 0,
                             _x   : PhantomData }
    }

    pub fn iter<'a>(&'a mut self) -> ISpineIter<'a, T> {
        ISpineIter { it: self.data().iter() }
    }

    pub fn iter_mut<'a>(&'a mut self) -> ISpineIterMut<'a, T> {
        ISpineIterMut { it: self.data_mut().iter_mut() }
    }
}

////////////////////////////////////////////////////////////

pub struct ISpineLocationIter<'a, T: 'a> {
    root : *mut ISpine<T>,
    idx  : usize,
    _x: PhantomData<&'a T>,
}

impl <'a, T: 'a> ISpineLocationIter<'a, T> {

    fn root(&self) -> &ISpine<T> {
        unsafe { &*self.root }
    }

    fn root_mut(&mut self) ->&mut  ISpine<T> {
        unsafe { &mut *self.root }
    }

}

pub struct ISpineLocation<'a, T: 'a> {
    root : *mut ISpine<T>,
    idx  : usize,
    _x   : PhantomData<&'a T>,
}

impl <'a, T: 'a> ISpineLocation<'a, T> {
    fn root(&self) -> &ISpine<T> {
        unsafe { &*self.root }
    }

    fn root_mut(&mut self) ->&mut  ISpine<T> {
        unsafe { &mut *self.root }
    }

}

impl <'a, T: 'a> Iterator for ISpineLocationIter<'a, T> {
    type Item = ISpineLocation<'a, T>;

    fn next(&mut self) -> Option<ISpineLocation<'a, T>> {
        let self_idx = self.idx;
        let self_len = self.root().data().len();
        if self_idx < self_len {
            let ret = ISpineLocation {
                root: self.root,
                idx : self.idx,
                _x  : PhantomData,
            };
            self.idx += 1;
            Some(ret)
        } else {
            None
        }
    }
}


impl <'a, T: 'a> ISpineLocation<'a, T> {
    pub fn insert(&mut self, x: T) {
        let self_idx = self.idx;
        self.root_mut().data_mut().insert(self_idx+1, x)
    }

    pub fn take_rest(&mut self) -> ISpine<T> {
        let self_idx = self.idx;
        self.root_mut().take_from(self_idx)
    }
}

////////////////////////////////////////////////////////////

impl <T> Default for ISpine<T> {
    fn default() -> ISpine<T> {
        ISpine { data: UnsafeCell::new(Vec::new()) }
    }
}

pub struct ISpineIterMut<'a, T: 'a> {
    it : IterMut<'a, T>,
}

impl <'a, T: 'a> Iterator for ISpineIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.it.next()
    }
}

pub struct ISpineIter<'a, T: 'a> {
    it : Iter<'a, T>,
}

impl <'a, T: 'a> Iterator for ISpineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.it.next()
    }
}

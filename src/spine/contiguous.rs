use std::iter::Iterator;
use std::slice::{Iter, IterMut};
use std::default::Default;
use std::mem;
use std::vec::*;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result};
use spine::{GiftSpine, GiftSpineLocation, GiftSpineLocationMut};
use std::ops::{Index, IndexMut};

pub struct Contiguous<T> {
    data : UnsafeCell<Vec<T>>,
}

impl <T: Debug> Debug for Contiguous<T> {
    fn fmt(&self, fmtr : &mut Formatter) -> Result {
        self.data().fmt(fmtr)
    }
}

impl <T: Clone> Clone for Contiguous<T> {
    fn clone(&self) -> Contiguous<T> {
        let inner_cln : Vec<T>= unsafe { &*self.data.get() }.clone();
        Contiguous { data: UnsafeCell::new(inner_cln) }
    }
}

impl <T: Debug> Contiguous<T> {
    #[inline(always)]
    fn data(&self) -> &Vec<T> {
        unsafe { &*self.data.get() }
    }

    #[inline(always)]
    fn data_mut(&mut self) ->&mut  Vec<T> {
        unsafe { &mut *self.data.get() }
    }

}

impl <'a, T : 'a + Debug> GiftSpine<'a> for Contiguous<T> {
    type T       = T;
    type Loc     = ContiguousLocation<'a, T>;
    type LocMut  = ContiguousLocationMut<'a, T>;
    type Iter    = ContiguousLocationIter<'a, T>;
    type MutIter = ContiguousLocationIterMut<'a, Self::T>;

    #[inline(always)]
    fn add(&mut self, x : T) {
        self.data_mut().insert(0, x)
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        if self.data().len() > 0 {
            Some(self.data_mut().remove(0))
        } else {
            None
        }
    }

    fn take(&mut self, n : usize) -> Contiguous<T> {
        let second = UnsafeCell::new(self.data_mut().split_off(n));
        let tmp = mem::replace(&mut self.data, second);
        Contiguous { data: tmp }
    }

    fn iter(&'a self) -> ContiguousLocationIter<'a, T> {
        ContiguousLocationIter { root: self,
                                 idx: 0,
                                 len: self.data().len(),
                                 _x: PhantomData }
    }

    fn iter_mut(&'a mut self) -> ContiguousLocationIterMut<'a, T> {
        ContiguousLocationIterMut { root: self,
                                    idx: 0,
                                    len: self.data().len(),
                                    _x: PhantomData }
    }

}

////////////////////////////////////////////////////////////

pub struct ContiguousLocationIter<'a, T: 'a> {
    root : *const Contiguous<T>,
    idx  : usize,
    len  : usize,
    _x: PhantomData<&'a T>,
}

pub struct ContiguousLocation<'a, T: 'a> {
    root : *const Contiguous<T>,
    idx  : usize,
    _x   : PhantomData<&'a T>,
}

impl <'a, T: 'a> ContiguousLocation<'a, T> {
    #[inline(always)]
    fn root(&self) -> &Contiguous<T> {
        unsafe { &*self.root }
    }
}

impl <'a, T: 'a> Iterator for ContiguousLocationIter<'a, T> {
    type Item = ContiguousLocation<'a, T>;

    fn next(&mut self) -> Option<ContiguousLocation<'a, T>> {
        let self_idx = self.idx;
        let self_len = self.len;
        if self_idx <= self_len {
            let ret = ContiguousLocation {
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.idx <= self.len {
            Some(ContiguousLocation {
                root : self.root,
                idx  : self.idx+n,
                _x   : PhantomData,
            })
        } else {
            None
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx <= self.len {
            Some(ContiguousLocation {
                root : self.root,
                idx  : self.len-1,
                _x   : PhantomData
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.len - self.idx - 1;
        (rem, Some(rem))
    }

    fn count(self) -> usize {
        self.len - self.idx
    }

}


impl <'a, T: 'a + Debug> GiftSpineLocation<T> for ContiguousLocation<'a, T> {
    type Spine = Contiguous<T>;

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.root().data().len() == 0
    }

    fn node(&self) -> &T {
        self.root().data().index(self.idx)
    }
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

pub struct ContiguousLocationIterMut<'a, T: 'a> {
    root : *mut Contiguous<T>,
    idx  : usize,
    len  : usize,
    _x: PhantomData<&'a T>,
}

pub struct ContiguousLocationMut<'a, T: 'a + Debug> {
    root : *mut Contiguous<T>,
    idx  : usize,
    _x   : PhantomData<&'a T>,
}

impl <'a, T: 'a + Debug> ContiguousLocationMut<'a, T> {
    #[inline(always)]
    fn root(&self) -> &Contiguous<T> {
        unsafe { &mut *self.root }
    }

    #[inline(always)]
    fn root_mut(&mut self) -> &mut  Contiguous<T> {
        unsafe { &mut *self.root }
    }
}

impl <'a, T: 'a + Debug> Iterator for ContiguousLocationIterMut<'a, T> {
    type Item = ContiguousLocationMut<'a, T>;

    fn next(&mut self) -> Option<ContiguousLocationMut<'a, T>> {
        let self_idx = self.idx;
        let self_len = self.len; //self.root().data().len();
        if self_idx < self_len {
            let ret = ContiguousLocationMut {
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.idx < self.len {
            Some(ContiguousLocationMut {
                root : self.root,
                idx  : self.idx+n,
                _x   : PhantomData,
            })
        } else {
            None
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx < self.len {
            Some(ContiguousLocationMut {
                root : self.root,
                idx  : self.len-1,
                _x   : PhantomData
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.len - self.idx - 1;
        (rem, Some(rem))
    }

    fn count(self) -> usize {
        self.len - self.idx - 1
    }

}


impl <'a, T: 'a + Debug> GiftSpineLocation<T> for ContiguousLocationMut<'a, T> {
    type Spine = Contiguous<T>;

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.root().data().len() == 0
    }

    fn node(&self) -> &T {
        self.root().data().index(self.idx)
    }
}

impl <'a, T: 'a + Debug> GiftSpineLocationMut<T> for ContiguousLocationMut<'a, T> {

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.root().data().len() == 0
    }

    fn node(&mut self) -> &mut T {
        let self_idx = self.idx;
        self.root_mut().data_mut().index_mut(self_idx)
    }

    fn insert(&mut self, x: T) {
        let self_idx = self.idx;
        self.root_mut().data_mut().insert(self_idx+1, x);
    }

    fn take(&mut self) -> Contiguous<T> {
        let self_idx = self.idx;
        let rest = UnsafeCell::new(self.root_mut().data_mut().split_off(self_idx));
        Contiguous { data: rest }
    }
}

////////////////////////////////////////////////////////////

impl <T> Default for Contiguous<T> {
    fn default() -> Contiguous<T> {
        Contiguous { data: UnsafeCell::new(Vec::new()) }
    }
}

pub struct ContiguousIterMut<'a, T: 'a> {
    it : IterMut<'a, T>,
}

impl <'a, T: 'a> Iterator for ContiguousIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.it.next()
    }
}

pub struct ContiguousIter<'a, T: 'a> {
    it : Iter<'a, T>,
}

impl <'a, T: 'a> Iterator for ContiguousIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.it.next()
    }
}

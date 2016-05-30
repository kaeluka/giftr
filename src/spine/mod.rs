use std::iter::Iterator;

pub trait GiftSpine<'a> : Default {
    type T : 'a;
    type LocIter : Iterator;
    type Iter    : Iterator<Item=&'a Self::T>;
    type MutIter : Iterator<Item=&'a mut Self::T>;

    fn is_null(&self) -> bool;
    fn add(&mut self, x : Self::T);
    fn pop(&mut self) -> Option<Self::T>;
    fn take(&mut self, n : usize) -> Self;
    fn take_from(&mut self, n : usize) -> Self;
    fn at(&'a mut self) -> Self::LocIter;
    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::MutIter;

}

pub trait GiftSpineLocation<T> {
    type Spine;
    fn insert(&mut self, x: T);
    fn take_rest(&mut self) -> Self::Spine;
}

pub mod contiguous;
//pub mod chunked;

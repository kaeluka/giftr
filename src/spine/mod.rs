use std::iter::Iterator;

//impl <'a, T: Clone, Dir: PartialEq> GiftTreeSpine<'a, Dir> for Ahnentafel<T> {
//    type Path = i32;
//}

pub trait GiftSpine<'a> : Default {
    type T       : 'a;
    type Loc     : GiftSpineLocation<Self::T> + 'a;
    type LocMut  : GiftSpineLocationMut<Self::T> + 'a;
    type Iter    : Iterator<Item=Self::Loc>;
    type MutIter : Iterator<Item=Self::LocMut>;

    fn add(&mut self, x : Self::T);        // would need a Dir
    fn pop(&mut self) -> Option<Self::T>;  // would need a Dir
    fn take(&mut self, n : usize) -> Self;
    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::MutIter;
}

pub trait GiftSpineLocation<T> {
    type Spine;
    fn node(&self) -> &T;
    fn is_null(&self) -> bool;             // would need a Dir
}

pub trait GiftSpineLocationMut<T> : GiftSpineLocation<T> {
    fn node(&mut self) -> &mut T;
    fn is_null(&self) -> bool;             // would need a Dir
    fn insert(&mut self, x: T);            // would need a Dir
    fn take(&mut self) -> Self::Spine;
}

pub mod contiguous;
//pub mod chunked;

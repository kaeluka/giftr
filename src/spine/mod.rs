#![macro_use]
use std::ops::{Deref};
use std::ops::{Index,IndexMut};

enum TraceSeg<'a, N: Node + 'a>  {
    This,
    Go(&'a mut N),
}

//type TracePath<'a, N: Node + 'a> = FnMut (&mut N) -> TraceSeg<'a, N>;

/// The root of a spine
pub trait Spine : Default + Node {
    type Node : Node<T=Self::T, Idx=Self::Idx>;
}

pub trait Node : Deref {
    type T;
    type Idx;

    fn put(&mut self, idx: Self::Idx, x: Self::T);

    fn pop(&mut self, idx: Self::Idx) -> Option<Self::T>;

    fn index(&self, idx: Self::Idx) -> &Self::T;

    fn index_mut(&mut self, idx: Self::Idx) -> &mut Self::T;

    fn is_null(&self) -> bool;

}

#[macro_export]
macro_rules! spine_impls {
    ( $t: ident ) => {
        impl <T> Deref for $t<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.index(0)
            }
        }

        impl <T> DerefMut for $t<T> {

            fn deref_mut(&mut self) -> &mut Self::Target {
                self.index_mut(0)
            }
        }

        impl <T> Index<usize> for $t<T> {
            type Output = T;
            fn index(&self, idx: usize) -> &Self::Output {
                self.index(idx)
            }
        }

        impl <T> IndexMut<usize> for $t<T> {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                self.index_mut(idx)
            }
        }

    };

}

pub mod contiguous;

#[cfg(test)]
mod tests {

    use spine::contiguous::Sp;

    #[test]
    fn foo() {

    }

}

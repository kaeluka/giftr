use std::marker::PhantomData;

#[derive(Debug)]
pub struct Ref<T> {
    _x : PhantomData<T>,
}


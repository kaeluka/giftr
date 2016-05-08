use std::ptr::copy_nonoverlapping;
use std::mem::{drop, zeroed};

//pub trait ShallowClone {
//    fn shallow_clone(&self) -> Self;
//    fn shallow_drop(Self);
//}
//
//impl <T: Clone> ShallowClone for T {
//    fn shallow_clone(&self) -> T {
//        self.clone()
//    }
//
//    fn shallow_drop(x:Self) {
//        drop(x);
//    }
//}

//pub unsafe fn brutal_shallow_clone<T:Clone>(x:&T) -> T {
//    let mut ret : T = zeroed();
//    copy_nonoverlapping(x, &mut ret, 1);
//    ret
//}

//pub unsafe fn shallow_clone_rc<T>(x:&Rc<T>) -> Rc<T> {
//    let mut ret : T = zeroed();
//    println!("{}", x.ptr.strong);
//    copy_nonoverlapping(x, &mut ret, 1);
//}

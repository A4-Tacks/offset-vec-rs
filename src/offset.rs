use alloc::{boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};

mod check;

pub use check::*;

use crate::{VecLike, OffsetVec};

#[track_caller]
pub fn create<V: VecLike>(vec: V, offset: usize) -> OffsetVec<V> {
    OffsetVec { vec, offset }.offset_check()
}

pub trait Offset {
    type Output: VecLike;
    type OutputMut: VecLike;

    fn offset(self, i: usize) -> OffsetVec<Self::Output>;

    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut>;
}

impl<'a, V: Offset + VecLike> Offset for &'a mut V {
    type Output = &'a mut V::OutputMut;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        (*self).offset_mut(i).offset_check()
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        (**self).offset_mut(i).offset_check()
    }
}

impl<V: Offset<Output = V> + VecLike> Offset for Box<V> {
    type Output = Self;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        (**self).offset_mut(i).offset_check()
    }
}

impl<V: Offset<Output = V> + VecLike + Clone> Offset for Rc<V> {
    type Output = Self;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        Self::make_mut(self).offset_mut(i).offset_check()
    }
}

impl<V: Offset<Output = V> + VecLike + Clone> Offset for Arc<V> {
    type Output = Self;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        Self::make_mut(self).offset_mut(i).offset_check()
    }
}


impl<V: Offset + VecLike> Offset for OffsetVec<V> {
    type Output = V::Output;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        self.vec.offset(self.offset+i).offset_check()
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        self.vec.offset_mut(self.offset+i).offset_check()
    }
}

impl<T> Offset for Vec<T> {
    type Output = Vec<T>;
    type OutputMut = Vec<T>;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        create(self, i)
    }
}

impl Offset for String {
    type Output = String;
    type OutputMut = String;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        let _ = &self[i..];
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        let _ = &self[i..];
        create(self, i)
    }
}

#[cfg(feature = "smallvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
impl<A: smallvec::Array> Offset for smallvec::SmallVec<A> {
    type Output = Self;
    type OutputMut = Self;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        create(self, i)
    }
}

#[cfg(feature = "smallstr")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallstr")))]
impl<A: smallvec::Array<Item = u8>> Offset for smallstr::SmallString<A> {
    type Output = Self;
    type OutputMut = Self;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        let _ = &self[i..];
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        let _ = &self[i..];
        create(self, i)
    }
}

#[cfg(feature = "rc-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "rc-vec")))]
impl<T> Offset for rc_vec::RcVec<T> {
    type Output = Self;
    type OutputMut = Self;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        create(self, i)
    }
}

#[cfg(feature = "rc-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "rc-vec")))]
impl<T> Offset for rc_vec::ArcVec<T> {
    type Output = Self;
    type OutputMut = Self;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::Output> {
        create(self, i)
    }
}

#[cfg(feature = "unique-rc")]
#[cfg_attr(docsrs, doc(cfg(feature = "unique-rc")))]
impl<V: Offset<Output = V> + VecLike> Offset for unique_rc::UniqRc<V> {
    type Output = Self;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        (**self).offset_mut(i).offset_check()
    }
}

#[cfg(feature = "unique-rc")]
#[cfg_attr(docsrs, doc(cfg(feature = "unique-rc")))]
impl<V: Offset<Output = V> + VecLike> Offset for unique_rc::UniqArc<V> {
    type Output = Self;
    type OutputMut = V::OutputMut;

    #[track_caller]
    fn offset(self, i: usize) -> OffsetVec<Self::Output> {
        create(self, i)
    }

    #[track_caller]
    fn offset_mut(&mut self, i: usize) -> OffsetVec<&mut Self::OutputMut> {
        (**self).offset_mut(i).offset_check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn multiple_offset() {
        let (vec, [o1, o2, o4]);
        vec = vec![1, 2, 3, 4];
        o1 = vec.offset(1);
        assert_eq!(o1.offset, 1);
        o2 = o1.offset(1);
        assert_eq!(o2.offset, 2);
        o4 = o2.offset(2);
        assert_eq!(o4.offset, 4);
    }

    #[test]
    fn multiple_offset_mut() {
        let (mut vec, [mut o1, mut o2, o4]);

        vec = vec![1, 2, 3, 4];
        o1 = vec.offset_mut(1);
        assert_eq!(o1.offset, 1);
        o2 = o1.offset_mut(1);
        assert_eq!(o2.offset, 2);
        o4 = o2.offset_mut(2);
        assert_eq!(o4.offset, 4);
    }

    #[test]
    #[should_panic = "5 out of length (is 4)"]
    fn checked_offset() {
        let vec = vec![1, 2, 3, 4];
        _ = vec.offset(5);
    }

    #[test]
    #[should_panic = "5 out of length (is 4)"]
    fn checked_multiple_offset() {
        let vec = vec![1, 2, 3, 4];
        _ = vec.offset(2).offset(3);
    }

    #[test]
    #[should_panic = "5 out of length (is 4)"]
    fn checked_offset_mut() {
        let mut vec = vec![1, 2, 3, 4];
        _ = vec.offset_mut(5);
    }


    #[test]
    #[should_panic = "5 out of length (is 4)"]
    fn checked_multiple_offset_mut() {
        let mut vec = vec![1, 2, 3, 4];
        _ = vec.offset_mut(2).offset_mut(3);
    }
}

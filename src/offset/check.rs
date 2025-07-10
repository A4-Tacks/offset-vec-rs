use alloc::{boxed::Box, vec::Vec};

use crate::{OffsetVec, VecLike};

pub trait OffsetCheckRef {
    #[inline]
    fn offset_check_ref(&self) {}
}

pub trait OffsetCheck: OffsetCheckRef + Sized {
    #[inline]
    #[track_caller]
    fn offset_check(self) -> Self {
        self.offset_check_ref();
        self
    }
}

impl<T: OffsetCheckRef> OffsetCheck for T {}

impl<T: OffsetCheckRef + ?Sized> OffsetCheckRef for &T {
    #[inline]
    #[track_caller]
    fn offset_check_ref(&self) {
        (**self).offset_check_ref();
    }
}
impl<T: OffsetCheckRef + ?Sized> OffsetCheckRef for &mut T {
    #[inline]
    #[track_caller]
    fn offset_check_ref(&self) {
        (**self).offset_check_ref();
    }
}
impl<T: OffsetCheckRef + ?Sized> OffsetCheckRef for Box<T> {
    #[inline]
    #[track_caller]
    fn offset_check_ref(&self) {
        (**self).offset_check_ref();
    }
}

impl<T> OffsetCheckRef for [T] {}
impl<T> OffsetCheckRef for Vec<T> {}
impl<V: VecLike> OffsetCheckRef for OffsetVec<V> {
    #[inline]
    #[track_caller]
    fn offset_check_ref(&self) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn assert_failed(offset: usize, len: usize) -> ! {
            panic!("offset {offset} out of length (is {len})");
        }

        let len = self.vec.len();
        let offset = self.offset;

        if offset > len {
            assert_failed(offset, len);
        }
    }
}
